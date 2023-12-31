use std::{
	collections::BTreeMap,
	cmp::Ord,
	sync::Arc
};
use arc_swap::ArcSwap;
use serde::{
	de::DeserializeOwned,
	Serialize
};
use anyhow::Result;

pub struct Database {
	inner_sled: ArcSwap<Option<sled::Db>>
}

impl Default for Database {
	fn default() -> Self {
		Self::new()
	}
}

impl Database {
	pub fn new() -> Self {
		Self {
			inner_sled: ArcSwap::new(Arc::new(None))
		}
	}

	pub fn init(&self) -> Result<()> {
		let sled_db = sled::open("data/db")?;
		self.inner_sled.swap(Arc::new(Some(sled_db)));

		Ok(())
	}

	/// Get a monotonic u64 ID, consistent across restarts. Not contiguous.
	pub fn get_monotonic_id(&self) -> u64 {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		db.generate_id().expect("failed to generate ID")
	}

	/// Read a single item from a `K->V` table.
	pub fn read_item<K: Into<u64>, V: DeserializeOwned>(&self, id: K, table: &[u8]) -> Result<Option<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;
		let value = table.get(id.into().to_be_bytes())?;

		if let Some(val) = value {
			let decoded: V = serde_cbor::from_slice(&val)?;
			Ok(Some(decoded))
		} else {
			Ok(None)
		}
	}

	/// Read many values at once from a `K->V` table.
	pub fn read_many<K: Into<u64>, V: DeserializeOwned>(&self, ids: Vec<K>, table: &[u8]) -> Result<Vec<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let mut values = Vec::new();

		for id in ids {
			let key = id.into().to_be_bytes();
			if let Some(val) = table.get(key)? {
				let decoded: V = serde_cbor::from_slice(&val)?;
				values.push(decoded);
			}
		}

		Ok(values)
	}

	/// Determine if an item by this key is present in the table.
	pub fn contains<K: Into<u64>>(&self, id: K, table: &[u8]) -> Result<bool> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;
		Ok(table.contains_key(id.into().to_be_bytes())?)
	}

	/// Determine if an item by this keypair is present in the table.
	pub fn contains_paired<K1: Into<u64>, K2: Into<u64>>(&self, id1: K1, id2: K2, table: &[u8]) -> Result<bool> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;
		let key = Self::generate_bikey(id1.into(), id2.into());

		Ok(table.contains_key(key)?)
	}

	/// Read all values out of a `K->V` table.
	pub fn read_all<V: DeserializeOwned>(&self, table: &[u8]) -> Result<Vec<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let mut values = Vec::new();

		for result in &table {
			let (_key, value) = result?;
			values.push(serde_cbor::from_slice(&value)?);
		}

		Ok(values)
	}

	/// Write an item to a `K->V` table.
	pub fn write_item<K: Into<u64>, V: Serialize>(&self, id: K, value: &V, table: &[u8]) -> Result<()> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let encoded = serde_cbor::to_vec(value)?;
		table.insert(id.into().to_be_bytes(), encoded)?;

		Ok(())
	}

	/// Delete an item from a `K->V` table.
	pub fn delete_item<K: Into<u64>, V: DeserializeOwned>(&self, id: K, table: &[u8]) -> Result<Option<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let key = id.into().to_be_bytes();
		// entry might not exist, propagate the None out while still handling the error
		Ok(table.remove(key)?.map(|v| serde_cbor::from_slice(&v)).transpose()?)
	}

	/// Read all keys from a `K->V` table.
	pub fn list_item<K: From<u64>>(&self, table: &[u8]) -> Result<Vec<K>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let mut ids = Vec::new();
		for res in &table {
			let (k, _v) = res?;
			let mut buf = [0; 8];
			buf.copy_from_slice(&k);
			ids.push(u64::from_be_bytes(buf).into())
		}

		Ok(ids)
	}

	/// Take two u64s and merge them into a single fixed-size byte array.
	fn generate_bikey(k1: u64, k2: u64) -> [u8; 16] {
		let mut key = [0; 16];
		key[0..8].copy_from_slice(&k1.to_be_bytes());
		key[8..16].copy_from_slice(&k2.to_be_bytes());
		key
	}

	/// Read an entry from a `(K1,K2)->V` two-key table.
	pub fn read_paired_item<K1: Into<u64>, K2: Into<u64>, V: DeserializeOwned>(&self, id1: K1, id2: K2, table: &[u8]) -> Result<Option<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let key = Self::generate_bikey(id1.into(), id2.into());

		let value = table.get(key)?;
		if let Some(val) = value {
			let decoded: V = serde_cbor::from_slice(&val)?;
			Ok(Some(decoded))
		} else {
			Ok(None)
		}
	}

	/// Write an entry to a `(K1,K2)->V` two-key table.
	pub fn write_paired_item<K1: Into<u64>, K2: Into<u64>, V: Serialize>(&self, id1: K1, id2: K2, value: V, table: &[u8]) -> Result<()> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let key = Self::generate_bikey(id1.into(), id2.into());

		let encoded = serde_cbor::to_vec(&value)?;
		table.insert(key, encoded)?;

		Ok(())
	}

	/// Delete an item from a `(K1,K2)->V` two-key table.
	pub fn delete_paired_item<K1: Into<u64>, K2: Into<u64>, V: DeserializeOwned>(&self, id1: K1, id2: K2, table: &[u8]) -> Result<Option<V>> {
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let key = Self::generate_bikey(id1.into(), id2.into());

		Ok(table.remove(key)?.map(|v| serde_cbor::from_slice(&v)).transpose()?)
	}

	/// Perform a `K1` prefix scan on a `(K1,K2)->V` structured table, returning `K2` and `V`.
	pub fn scan_items_by_prefix<K1, K2, V>(&self, id1: K1, table: &[u8]) -> Result<BTreeMap<K2, V>>
	where
		K1: Into<u64>,
		K2: From<u64> + Ord,
		V: DeserializeOwned
	{
		let db = self.inner_sled.load();
		let db = db.as_ref().as_ref().expect("database was not loaded");
		let table = db.open_tree(table)?;

		let key = id1.into().to_be_bytes();

		let mut ret = BTreeMap::new();
		for item in table.scan_prefix(key) {
			let (composite_key, value) = item?;
			let mut buf = [0; 8];
			buf.copy_from_slice(&composite_key[8..16]);

			let right_key: K2 = u64::from_be_bytes(buf).into();
			let value: V = serde_cbor::from_slice(&value)?;
			ret.insert(right_key, value);
		}

		Ok(ret)
	}
}
