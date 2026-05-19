use crate::error::TripError;
use std::fmt::Debug;

/// A one-time use container for sensitive values
/// like otp (one-time password) i prefer to say oto (one time object)
/// , authenticators, etc...
/// # Important
/// - `Clone` is **intentionally not implemented** to prevent duplicate access
/// - `Copy` is impossible due to the internal state
/// - Once the value is extracted, the fuse becomes `Burnt`
/// - inner structures are not accessible to public users
/// - accessibility is provided with API methods
/// - in `new` method stick with owned values `T`, references are not allowed `&T`
/// one special exception is &`static str
/// 
#[derive(Debug)]
pub enum OnceFuse<T> {
    /// `FuseState<T>` is private type,not accessible to public API  
    /// # variants : knowledge purpose only, you can't use them
    /// - `Armed(T)` the fuse is still active,never used before
    /// - `Burnt` the fuse has burnt due to one-time use
    /// - `ExplicitBurnt` the fuse has burnt due to explicit call to `&mut self.burn_it()`
    #[allow(private_interfaces)]
    State(FuseState<T>),
}

///enumerates fuse states
#[derive(Debug)]
enum FuseState<T> {
    Armed(T),
    Burnt,
    ExplicitBurnt,
}

impl<T: 'static> OnceFuse<T> {
    /// Wraps and takes ownership of oto (one-time object) and returns a
    /// `OnceFuse` wrapper that guarantees that the value will cannot be used
    /// more than once
    /// # usage
    /// ```rust
    /// use tripfuse::OnceFuse;
    /// let secret="123abc";
    /// let mut fuse=OnceFuse::new(secret);
    /// assert_eq!(
    ///     fuse.try_use().unwrap(), "123abc"
    ///     )
    /// ```
    ///
    /// this example will not compile because of reference is not `static
    /// values stored in `OnceFuse` must moves the ownership to inside the fuse
    /// so we added `static to ensure (99%) that you can not use references
    /// because of the generic parameter `T` can take `&T`, the most idiomaticic exisiting solution
    /// to use `static which eliminates almost all references with the exception of &`static str which is `static
    ///
    /// # this will faile to compile  
    /// ```compile_fail
    /// use tripfuse::OnceFuse;
    /// let secret=String::from("123");
    /// let _fuse=OnceFuse::new(&secret);
    /// ```
    ///
    /// while this segment will compile
    /// ```rust
    /// use tripfuse::OnceFuse;
    /// let secret:&'static str="123abc";
    /// let _fuse=OnceFuse::new(secret);
    /// ```
    #[inline]
    pub fn new(oto: T) -> Self
    where
        T:
    {
        Self::State(FuseState::Armed(oto))
    }

    /// # case 1
    /// tries to use the inner value and return it as owned `T`
    /// (moves out of it) if never used before
    /// ```rust
    /// use tripfuse::OnceFuse;
    /// let secret=vec![1,2,3];
    /// let mut fuse=OnceFuse::new(secret);
    /// if let Ok(oto)=fuse.try_use(){
    ///     assert!(true);
    /// }else{
    ///     assert!(false);
    /// }
    /// ```
    ///
    /// # case 2
    /// returns `Err(TripError::FuseBurntAfterUsage)` if the fuse already used once
    /// ```rust
    /// use tripfuse::{OnceFuse,TripError};
    /// let secret="512402";
    /// let mut fuse= OnceFuse::new(secret);
    ///
    /// //fuse will be burnt here
    /// let _=fuse.try_use();
    /// //try to use again
    /// match fuse.try_use(){
    ///     Err(e)=>{
    ///         if let TripError::FuseBurntAfterUsage=e{
    ///             assert!(true);
    ///         }else{
    ///             assert!(false);
    ///         }
    ///     },
    ///     _=>{
    ///     assert!(false);
    ///     }
    /// }
    /// ```
    ///
    /// # case 3
    /// returns `Err(TripError::FuseBurntExplicitly)` if you explicitly called
    /// fuse.burn_it()
    /// ```rust
    /// use tripfuse::{OnceFuse,TripError};
    /// let secret=[0;11];
    /// let mut fuse=OnceFuse::new(secret);
    ///
    /// //returns true if the fuse burnt due to this call
    /// //false otherwise
    /// let burnt=fuse.burn_it();
    /// assert_eq!(burnt,true);
    ///
    /// //examine the fuse
    /// match fuse.try_use(){
    ///     Ok(_)=>{
    ///     assert!(false);
    ///     },
    ///     Err(e)=>{
    ///        match e{
    ///         TripError::FuseBurntAfterUsage=>{assert!(false)},
    ///         TripError::FuseBurntExplicitly=>{assert!(true)},
    ///         _=>{assert!(false)}
    ///         }
    ///     }
    ///}
    /// ```
    ///
    /// # case 4
    /// if fuse burnt due to usage and we later called fuse.burn_it() what will happen?
    /// actually nothing, no value will be yielded,and fuse will stays in State(FuseState::Burnt),
    /// which acts as case 2
    /// ```rust
    /// use tripfuse::{OnceFuse,TripError};
    /// let secret=[1,2,3];
    /// let mut fuse=OnceFuse::new(secret);
    /// let _=fuse.try_use(); //fuse is burnt after this call
    ///
    /// //try to explicit burn while the current state is State(FuseState::Burnt)
    /// let exp_burnt=fuse.burn_it();
    /// assert_eq!(exp_burnt,false);
    ///
    /// //examine
    /// if let Err(TripError::FuseBurntAfterUsage)=fuse.try_use(){
    ///     assert!(true);
    /// }else{
    ///     assert!(false);
    /// }
    /// ```
    ///
    #[must_use = "This returns the inner value or an error. Handle the result appropriately."]
    pub fn try_use(&mut self) -> Result<T, TripError> {
        let current = std::mem::replace(self, Self::State(FuseState::Burnt));

        match current {
            Self::State(FuseState::Armed(value)) => Ok(value),
            Self::State(FuseState::Burnt) => Err(TripError::FuseBurntAfterUsage),
            Self::State(FuseState::ExplicitBurnt) => Err(TripError::FuseBurntExplicitly),
        }
    }

    /// explicitly burns the fuse
    /// return true if fuse burnt due to this method call, false otherwise (if burnt due to usage)
    /// but ultimately the fuse will be burnt in State(FuseState::Burnt) or State(FuseState::ExplicitBurnt)
    /// which ensures the fuse is not usable again
    ///
    /// # case 1 return true
    /// ```
    /// use tripfuse::{OnceFuse,TripError};
    /// let secret=String::from("123abc");
    /// let mut fuse=OnceFuse::new(secret);
    /// let exp_burnt=fuse.burn_it();
    /// assert_eq!(exp_burnt,true);
    ///
    /// //examine
    /// if let Err(TripError::FuseBurntExplicitly)=fuse.try_use(){
    ///     assert!(true);
    /// }else{
    ///     assert!(false);
    /// }
    /// ```
    ///
    /// # case 2
    /// returns false if fuse used before
    /// ```
    /// use tripfuse::{OnceFuse,TripError};
    /// let secret=String::from("123abc");
    /// let mut fuse=OnceFuse::new(secret);
    /// let _=fuse.try_use();
    /// let exp_burnt=fuse.burn_it();
    /// assert_eq!(exp_burnt,false);
    ///
    /// //examine
    /// if let Err(TripError::FuseBurntAfterUsage)=fuse.try_use(){
    ///     assert!(true);
    /// }else{
    ///     assert!(false);
    /// }
    /// ```
    pub fn burn_it(&mut self) -> bool {
        if let Self::State(FuseState::Armed(_)) = self {
            let _ = std::mem::replace(self, OnceFuse::State(FuseState::ExplicitBurnt));
            true
        } else {
            false
        }
    }

    pub fn is_armed(&self) -> bool {
        matches!(self, OnceFuse::State(FuseState::Armed(_)))
    }

    pub fn is_burnt(&self) -> bool {
        !self.is_armed()
    }
}
