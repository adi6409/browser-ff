//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Slot.idl
//


/// `interface nsIPKCS11Slot : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPKCS11Slot {
    vtable: *const nsIPKCS11SlotVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPKCS11Slot.
unsafe impl XpCom for nsIPKCS11Slot {
    const IID: nsIID = nsID(0xc2d4f296, 0xee60, 0x11d4,
        [0x99, 0x8b, 0x00, 0xb0, 0xd0, 0x23, 0x54, 0xa0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPKCS11Slot {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPKCS11Slot.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPKCS11SlotCoerce {
    /// Cheaply cast a value of this type from a `nsIPKCS11Slot`.
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPKCS11SlotCoerce for nsIPKCS11Slot {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self {
        v
    }
}

impl nsIPKCS11Slot {
    /// Cast this `nsIPKCS11Slot` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPKCS11SlotCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPKCS11Slot {
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
impl<T: nsISupportsCoerce> nsIPKCS11SlotCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Slot) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPKCS11Slot
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPKCS11SlotVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String desc; */
    pub GetDesc: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aDesc: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String manID; */
    pub GetManID: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aManID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String HWVersion; */
    pub GetHWVersion: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aHWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String FWVersion; */
    pub GetFWVersion: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aFWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aStatus: *mut u32) -> ::nserror::nsresult,

    /* [must_use] nsIPK11Token getToken (); */
    pub GetToken: unsafe extern "system" fn (this: *const nsIPKCS11Slot, _retval: *mut*const nsIPK11Token) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String tokenName; */
    pub GetTokenName: unsafe extern "system" fn (this: *const nsIPKCS11Slot, aTokenName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPKCS11Slot {

    pub const SLOT_DISABLED: i64 = 0;


    pub const SLOT_NOT_PRESENT: i64 = 1;


    pub const SLOT_UNINITIALIZED: i64 = 2;


    pub const SLOT_NOT_LOGGED_IN: i64 = 3;


    pub const SLOT_LOGGED_IN: i64 = 4;


    pub const SLOT_READY: i64 = 5;


    /// `[must_use] readonly attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `[must_use] readonly attribute AUTF8String desc;`
    #[inline]
    pub unsafe fn GetDesc(&self, aDesc: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDesc)(self, aDesc)
    }


    /// ```text
    /// /**
    ///    * Manufacturer ID of the slot.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String manID;`
    #[inline]
    pub unsafe fn GetManID(&self, aManID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetManID)(self, aManID)
    }


    /// ```text
    /// /**
    ///    * Hardware version of the slot.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String HWVersion;`
    #[inline]
    pub unsafe fn GetHWVersion(&self, aHWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHWVersion)(self, aHWVersion)
    }


    /// ```text
    /// /**
    ///    * Firmware version of the slot.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String FWVersion;`
    #[inline]
    pub unsafe fn GetFWVersion(&self, aFWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFWVersion)(self, aFWVersion)
    }



    /// `[must_use] readonly attribute unsigned long status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }



    /// `[must_use] nsIPK11Token getToken ();`
    #[inline]
    pub unsafe fn GetToken(&self, _retval: *mut*const nsIPK11Token) -> ::nserror::nsresult {
        ((*self.vtable).GetToken)(self, _retval)
    }



    /// `[must_use] readonly attribute AUTF8String tokenName;`
    #[inline]
    pub unsafe fn GetTokenName(&self, aTokenName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenName)(self, aTokenName)
    }


}


