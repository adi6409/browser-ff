//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIDomainPolicy.idl
//


/// `interface nsIDomainPolicy : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDomainPolicy {
    vtable: *const nsIDomainPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDomainPolicy.
unsafe impl XpCom for nsIDomainPolicy {
    const IID: nsIID = nsID(0x82b24a20, 0x6701, 0x4d40,
        [0xa0, 0xf9, 0xf5, 0xdc, 0x73, 0x21, 0xb5, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDomainPolicy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDomainPolicy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDomainPolicyCoerce {
    /// Cheaply cast a value of this type from a `nsIDomainPolicy`.
    fn coerce_from(v: &nsIDomainPolicy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDomainPolicyCoerce for nsIDomainPolicy {
    #[inline]
    fn coerce_from(v: &nsIDomainPolicy) -> &Self {
        v
    }
}

impl nsIDomainPolicy {
    /// Cast this `nsIDomainPolicy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDomainPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDomainPolicy {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIDomainPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDomainPolicy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDomainPolicy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDomainPolicyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIDomainSet blocklist; */
    pub GetBlocklist: unsafe extern "system" fn (this: *const nsIDomainPolicy, aBlocklist: *mut*const nsIDomainSet) -> ::nserror::nsresult,

    /* readonly attribute nsIDomainSet superBlocklist; */
    pub GetSuperBlocklist: unsafe extern "system" fn (this: *const nsIDomainPolicy, aSuperBlocklist: *mut*const nsIDomainSet) -> ::nserror::nsresult,

    /* readonly attribute nsIDomainSet allowlist; */
    pub GetAllowlist: unsafe extern "system" fn (this: *const nsIDomainPolicy, aAllowlist: *mut*const nsIDomainSet) -> ::nserror::nsresult,

    /* readonly attribute nsIDomainSet superAllowlist; */
    pub GetSuperAllowlist: unsafe extern "system" fn (this: *const nsIDomainPolicy, aSuperAllowlist: *mut*const nsIDomainSet) -> ::nserror::nsresult,

    /* void deactivate (); */
    pub Deactivate: unsafe extern "system" fn (this: *const nsIDomainPolicy) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
    /// Unable to generate binding because `native type mozilla::dom::DomainPolicyClone unsupported`
    pub CloneDomainPolicy: *const ::libc::c_void,

    /* [noscript,notxpcom] void applyClone (in DomainPolicyCloneConstPtr aClone); */
    /// Unable to generate binding because `native type const mozilla::dom::DomainPolicyClone unsupported`
    pub ApplyClone: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDomainPolicy {


    /// `readonly attribute nsIDomainSet blocklist;`
    #[inline]
    pub unsafe fn GetBlocklist(&self, aBlocklist: *mut*const nsIDomainSet) -> ::nserror::nsresult {
        ((*self.vtable).GetBlocklist)(self, aBlocklist)
    }



    /// `readonly attribute nsIDomainSet superBlocklist;`
    #[inline]
    pub unsafe fn GetSuperBlocklist(&self, aSuperBlocklist: *mut*const nsIDomainSet) -> ::nserror::nsresult {
        ((*self.vtable).GetSuperBlocklist)(self, aSuperBlocklist)
    }



    /// `readonly attribute nsIDomainSet allowlist;`
    #[inline]
    pub unsafe fn GetAllowlist(&self, aAllowlist: *mut*const nsIDomainSet) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowlist)(self, aAllowlist)
    }



    /// `readonly attribute nsIDomainSet superAllowlist;`
    #[inline]
    pub unsafe fn GetSuperAllowlist(&self, aSuperAllowlist: *mut*const nsIDomainSet) -> ::nserror::nsresult {
        ((*self.vtable).GetSuperAllowlist)(self, aSuperAllowlist)
    }



    /// `void deactivate ();`
    #[inline]
    pub unsafe fn Deactivate(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Deactivate)(self, )
    }



    /// `[noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone);`
    const _CloneDomainPolicy: () = ();


    /// `[noscript,notxpcom] void applyClone (in DomainPolicyCloneConstPtr aClone);`
    const _ApplyClone: () = ();

}


/// `interface nsIDomainSet : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDomainSet {
    vtable: *const nsIDomainSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDomainSet.
unsafe impl XpCom for nsIDomainSet {
    const IID: nsIID = nsID(0x665c981b, 0x0a0f, 0x4229,
        [0xac, 0x06, 0xa8, 0x26, 0xe0, 0x2d, 0x4f, 0x69]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDomainSet {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDomainSet.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDomainSetCoerce {
    /// Cheaply cast a value of this type from a `nsIDomainSet`.
    fn coerce_from(v: &nsIDomainSet) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDomainSetCoerce for nsIDomainSet {
    #[inline]
    fn coerce_from(v: &nsIDomainSet) -> &Self {
        v
    }
}

impl nsIDomainSet {
    /// Cast this `nsIDomainSet` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDomainSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDomainSet {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIDomainSetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDomainSet) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDomainSet
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDomainSetVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void add (in nsIURI aDomain); */
    pub Add: unsafe extern "system" fn (this: *const nsIDomainSet, aDomain: *const nsIURI) -> ::nserror::nsresult,

    /* void remove (in nsIURI aDomain); */
    pub Remove: unsafe extern "system" fn (this: *const nsIDomainSet, aDomain: *const nsIURI) -> ::nserror::nsresult,

    /* void clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsIDomainSet) -> ::nserror::nsresult,

    /* bool contains (in nsIURI aDomain); */
    pub Contains: unsafe extern "system" fn (this: *const nsIDomainSet, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool containsSuperDomain (in nsIURI aDomain); */
    pub ContainsSuperDomain: unsafe extern "system" fn (this: *const nsIDomainSet, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDomainSet {


    /// `void add (in nsIURI aDomain);`
    #[inline]
    pub unsafe fn Add(&self, aDomain: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).Add)(self, aDomain)
    }



    /// `void remove (in nsIURI aDomain);`
    #[inline]
    pub unsafe fn Remove(&self, aDomain: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, aDomain)
    }



    /// `void clear ();`
    #[inline]
    pub unsafe fn Clear(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, )
    }



    /// `bool contains (in nsIURI aDomain);`
    #[inline]
    pub unsafe fn Contains(&self, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Contains)(self, aDomain, _retval)
    }



    /// `bool containsSuperDomain (in nsIURI aDomain);`
    #[inline]
    pub unsafe fn ContainsSuperDomain(&self, aDomain: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ContainsSuperDomain)(self, aDomain, _retval)
    }


}


