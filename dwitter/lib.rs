#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::Mapping;
#[ink::contract]
mod dwitter {
    // use ink_env::AccountId;

    use ink_storage::Mapping;
    use scale::{Encode, Decode};


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Dwitter {
        /// Stores a single `bool` value on the storage.
        value: bool,
        user:User,
        totalDweet:i128,
        dweets:Mapping<i128,Dweet>,
        dweetLikers:Mapping<i128,AccountId>
    }
#[derive(Debug)]    
    
    enum accountStatus{NP,Active,Banned,Deactivated}
      
    

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
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
    
    
    
    #[derive(ink_storage::traits::SpreadAllocate, ink_storage::traits::SpreadLayout,ink_storage::traits::PackedLayout,Encode,Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Dweet {
        dweetId:i128,
        author:AccountId,
        hashtag:String,
        content:String,
        imgHash:String,
        timestamp:i128,
        likecount:i128,
        reportCount:i128,
        // status:cdStatus

    }

    pub struct Comment {
        commentId:i128,
        author:AccountId,
        dweetId:i128,
        content:String,
        likeCount:i128,
        
    }

    #[ink(event)]
    pub struct DweetCreated{
        id:i128
    }

    #[ink(event)] 
    pub struct DweetDeleted {
        id:i128
    }



    #[ink(event)]
    pub struct DweetLiked{
        id:i128,
        liker:AccountId
    }

    impl Dwitter {

        
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool,username:String,name:String,bio:String) -> Self {
             let mut dweets = Mapping::default();
             let mut dweetLikers = Mapping::default();
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
            Self { value: init_value ,user,totalDweet:0,dweets,dweetLikers:dweetLikers}
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


        #[ink(message)]
       pub fn createDweet(&self , hashtag:String,content:String,imghash:String) {
              self.totalDweet.checked_add(1);
              let id = self.totalDweet;
              let caller = self.env().caller();
            let  dweet = Dweet{
                dweetId:id,
                author:caller,
                hashtag,
                content,
                imgHash:imghash,
                timestamp:0,
                likecount:0,
                reportCount:0
              };
              self.dweets.insert(id, &dweet); 

              self.env().emit_event( DweetCreated{ id });

               
       }

       #[ink(message)]
       pub fn deleteDweet(&self ,id:i128){
           self.dweets.remove(id);
           self.env().emit_event(DweetDeleted{id})

       }
   
       #[ink(message)]
       pub fn likeDweet(&self,id:i128){
         let caller = self.env().caller(); 
        let mut dweet =self.dweets.get(id).unwrap();
        dweet.likecount.checked_add(1);
        self.dweetLikers.insert(id, &caller);
        self.env().emit_event(DweetLiked{id:id,liker:caller})


       }

       #[ink(message)]
       pub fn create_comment(&self,id:i128,comment:String){
        // let mut dweet = self.dweets.get(id).unwrap();
        // dweet.c

       }

       #[ink(message)]
       pub fn getDweet(&self,id:i128) -> Dweet {

       return  self.dweets.get(id).unwrap();


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
            let mut dwitter = Dwitter::new(false,String::from("user1"),String::from("username"),String::from("userbio"));
            assert_eq!(dwitter.get(), false);
            dwitter.flip();
            assert_eq!(dwitter.get(), true);
        }
    }
}
