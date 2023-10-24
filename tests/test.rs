use crate::person::{default_ssn};
use internal::internal;

#[internal]
mod person {
	use internal::internal;

	mod inner {
		static DEEP: &'static str = "OHYES";
	}

	pub fn default_ssn() -> &'static str {
		"555-55-5555"
	}

	/// Makes a person.
	fn person_maker() -> Person {
		Person {
			name: "John Doe",
			ssn: default_ssn()
		}
	}


	struct Person {
		pub name: &'static str,
		ssn: &'static str,
	}

	pub struct PublicPerson(Person);


	impl PublicPerson {
		fn new() -> PublicPerson {
			PublicPerson(Person {
				name: "John Doe",
				ssn: default_ssn(),
			})
		}

		fn ssn(&self) -> &'static str {
			self.0.ssn
		}
	}
}

#[test]
fn private_struct_compiles() {
	let person = person::Person {
		name: "John Doe",
		ssn: default_ssn()
	};
	assert_eq!(person.ssn, default_ssn());
}

#[test]
fn public_struct_compiles() {
	let person = person::PublicPerson(person::Person {
		name: "John Doe",
		ssn: default_ssn()
	});
	assert_eq!(person.ssn(), default_ssn());
}

#[test]
fn fn_compiles() {
	let person = person::person_maker();
	assert_eq!(person.ssn, default_ssn());
}
