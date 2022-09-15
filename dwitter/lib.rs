#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::Mapping;
#[ink::contract]
mod dwitter {
    // use ink_env::AccountId;

    use ink_storage::Mapping;
    use scale::Encode;


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dwitter {
        /// Stores a single `bool` value on the storage.
        value: bool,
        user:User
    }
    
#[derive(Debug)]    
    
    enum accountStatus{NP,Active,Banned,Deactivated}
    #[derive(Debug,Encode)]
    enum cdStatus{NP,Active, Banned, Deleted}//Comment-Dweet status

    
#[derive(Debug)]    
      pub struct User{
        id :i64,
        address: AccountId,
         username:String,
         name:String,
         profileImgHash:String,
         profileCoverImgHash:String,
         bio:String,
        status :accountStatus // Account Banned or Not
    }

    impl ink_storage::traits::SpreadLayout for User {
        const FOOTPRINT: u64 = 1;
    
        fn pull_spread(ptr: &mut ink_primitives::KeyPtr) -> Self {
            Self { id: ink_storage::traits::SpreadLayout::pull_spread(ptr), address: todo!(), username: todo!(), name: todo!(), profileImgHash: todo!(), profileCoverImgHash: todo!(), bio: todo!(), status: todo!() }
        }
    
        fn push_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
            ink_storage::traits::SpreadLayout::push_spread(&self.id, ptr);
        }
    
        fn clear_spread(&self, ptr: &mut ink_primitives::KeyPtr) {
            ink_storage::traits::SpreadLayout::clear_spread(&self.id, ptr);
        }
    }
    
    
    
    pub struct Dweet {
        dweetId:i128,
        author:AccountId,
        hashtag:String,
        content:String,
        imgHash:String,
        timestamp:i128,
        likecount:i128,
        reportCount:i128,
        status:cdStatus

    }


    impl Dwitter {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool,username:String,name:String,bio:String) -> Self {
            // let mut users = Mapping::default();
            let caller = Self::env().caller();

            let user = User {
                id:0,
               address:caller,
               username,
               name:username,
               profileImgHash:String::from(""),
               profileCoverImgHash:String::from(""),
               bio,
               status:accountStatus::Active


                
            };
            // users.insert(caller, &user);
            Self { value: init_value ,user}
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(),String::from("user1"),String::from("username"),String::from("userbio"))
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let dwitter = Dwitter::default();
            assert_eq!(dwitter.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut dwitter = Dwitter::new(false);
            assert_eq!(dwitter.get(), false);
            dwitter.flip();
            assert_eq!(dwitter.get(), true);
        }
    }
}
