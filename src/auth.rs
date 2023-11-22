use crate::prelude::*;
use rocket::request::{self, Request, FromRequest};

pub struct User {
	username: String,
	password_hash: u64,
	kind: UserKind
}

pub enum UserKind {
	Customer,
	Admin,
	Receptionist,
}

#[post("/login")]
pub fn login() {

}

#[post("/logout")]
pub fn logout() {

}

pub struct Admin(());

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		let db = req.rocket().state::<crate::database::Database>();
		todo!()
	}
}

pub struct Receptionist(());

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Receptionist {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		todo!()
	}
}

pub struct Customer(());

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Customer {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
		todo!()
	}
}
