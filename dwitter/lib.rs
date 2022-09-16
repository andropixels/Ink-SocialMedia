#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;




#[ink::contract]
mod dwitter {
    // use ink_env::AccountId;

    use ink_storage::Mapping;
    // use ink::crates::prelude::{
    //     string::{
    //         String,
    //         ToString,
    //     },
    //     vec::Vec,
    // };
    use ink_prelude::string::String; 
    use scale::{Encode, Decode};


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(ink_storage::traits::SpreadAllocate)]
    pub struct Dwitter {
        /// Stores a single `bool` value on the storage.
        value: bool,
        user:User,
        totalDweet:i128,
        dweets:Mapping<i128,Dweet>,
        dweetLikers:Mapping<i128,AccountId>,
        comments:Mapping<AccountId,Comment>,
        test:i32
    }
    #[derive( ink_storage::traits::SpreadLayout,ink_storage::traits::PackedLayout,Encode,Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    
    enum accountStatus{NP,Active,Banned,Deactivated}
      
    

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    enum cdStatus{NP,Active, Banned, Deleted}//Comment-Dweet status

    
    #[derive(ink_storage::traits::StorageLayout,ink_storage::traits::SpreadAllocate, ink_storage::traits::SpreadLayout,ink_storage::traits::PackedLayout,Encode,Decode,Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
      pub struct User{
        id :i64,
        address: AccountId,
         username:String,
         name:String,
         profileImgHash:String,
         profileCoverImgHash:String,
         bio:String,
        // Account Banned or Not
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
        commentCount:i128,
        reportCount:i128,
        // status:cdStatus

    }
    #[derive(ink_storage::traits::SpreadAllocate, ink_storage::traits::SpreadLayout,ink_storage::traits::PackedLayout,Encode,Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Comment {
        commentId:i128,
        author:AccountId,
        dweetId:i128,
        content:String,
        likeCount:i128,
        timestamp:i128,

        
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

            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                
            });
          
             let mut dweets = Mapping::default();
             let mut dweetLikers = Mapping::default();
             let mut comments = Mapping::default();

            let caller = Self::env().caller();

            let user = User {
                id:0,
               address:caller,
               username,
               name:name,
               profileImgHash:String::from(""),
               profileCoverImgHash:String::from(""),
               bio,
               


                
            };
            // users.insert(caller, &user);
            
            Self { value: init_value ,user,totalDweet:0,dweets,dweetLikers:dweetLikers,comments,test:0}
            
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            //  Self::new(Default::default(),String::from("user1"),String::from("username"),String::from("userbio"))
            ink_lang::utils::initialize_contract(|_| {})
            
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
       pub fn createDweet(&mut self , hashtag:String,content:String,imghash:String) {
              self.totalDweet.checked_add(1);
              let id = self.totalDweet;
              let caller = self.env().caller();
            let mut  dweet = Dweet{
                dweetId:id,
                author:caller,
                hashtag,
                content,
                imgHash:imghash,
                timestamp:0,
                likecount:0,
                commentCount:0,
                reportCount:0
              };
              self.test.checked_add(1);
              self.dweets.insert(id, &dweet); 

              self.env().emit_event( DweetCreated{ id });

               
       }

       #[ink(message)]
       pub fn deleteDweet(&self ,id:i128){
           self.dweets.remove(id);
           self.env().emit_event(DweetDeleted{id})

       }
   
       #[ink(message)]
       pub fn likeDweet(&mut self,id:i128){
         let caller = self.env().caller(); 
        let mut dweet =self.dweets.get(id).unwrap();
        dweet.likecount.checked_add(1);
        self.dweetLikers.insert(id, &caller);
        self.env().emit_event(DweetLiked{id:id,liker:caller})


       }

       #[ink(message)]
       pub fn create_comment(&mut self,id:i128,comment:String){
        // let mut dweet = self.dweets.get(id).unwrap();
        // dweet.c
        // let mut comment = self.comments.get
        let caller = self.env().caller();
        let mut comment = Comment {
            commentId:0,
            author:caller,
            dweetId:id,
            content:comment,
            likeCount:0,
            timestamp:0


        };


      self.comments.insert(caller, &comment);
      let mut dweet = self.dweets.get(id).unwrap();
      dweet.commentCount.checked_add(1);

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
