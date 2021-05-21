//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIDeviceSensors.idl
//


/// `interface nsIDeviceSensorData : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDeviceSensorData {
    vtable: *const nsIDeviceSensorDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDeviceSensorData.
unsafe impl XpCom for nsIDeviceSensorData {
    const IID: nsIID = nsID(0x0462247e, 0xfe8c, 0x4aa5,
        [0xb6, 0x75, 0x37, 0x52, 0x54, 0x7e, 0x48, 0x5f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDeviceSensorData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDeviceSensorData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDeviceSensorDataCoerce {
    /// Cheaply cast a value of this type from a `nsIDeviceSensorData`.
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDeviceSensorDataCoerce for nsIDeviceSensorData {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self {
        v
    }
}

impl nsIDeviceSensorData {
    /// Cast this `nsIDeviceSensorData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDeviceSensorDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDeviceSensorData {
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
impl<T: nsISupportsCoerce> nsIDeviceSensorDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensorData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDeviceSensorData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDeviceSensorDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIDeviceSensorData, aType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute double x; */
    pub GetX: unsafe extern "system" fn (this: *const nsIDeviceSensorData, aX: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double y; */
    pub GetY: unsafe extern "system" fn (this: *const nsIDeviceSensorData, aY: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double z; */
    pub GetZ: unsafe extern "system" fn (this: *const nsIDeviceSensorData, aZ: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDeviceSensorData {

    pub const TYPE_ORIENTATION: i64 = 0;


    pub const TYPE_ACCELERATION: i64 = 1;


    pub const TYPE_PROXIMITY: i64 = 2;


    pub const TYPE_LINEAR_ACCELERATION: i64 = 3;


    pub const TYPE_GYROSCOPE: i64 = 4;


    pub const TYPE_LIGHT: i64 = 5;


    pub const TYPE_ROTATION_VECTOR: i64 = 6;


    pub const TYPE_GAME_ROTATION_VECTOR: i64 = 7;


    /// `readonly attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute double x;`
    #[inline]
    pub unsafe fn GetX(&self, aX: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetX)(self, aX)
    }



    /// `readonly attribute double y;`
    #[inline]
    pub unsafe fn GetY(&self, aY: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetY)(self, aY)
    }



    /// `readonly attribute double z;`
    #[inline]
    pub unsafe fn GetZ(&self, aZ: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetZ)(self, aZ)
    }


}


/// `interface nsIDeviceSensors : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDeviceSensors {
    vtable: *const nsIDeviceSensorsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDeviceSensors.
unsafe impl XpCom for nsIDeviceSensors {
    const IID: nsIID = nsID(0xe46e47c7, 0x55ff, 0x44c4,
        [0xab, 0xce, 0x21, 0xb1, 0x4b, 0xa0, 0x7f, 0x86]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDeviceSensors {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDeviceSensors.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDeviceSensorsCoerce {
    /// Cheaply cast a value of this type from a `nsIDeviceSensors`.
    fn coerce_from(v: &nsIDeviceSensors) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDeviceSensorsCoerce for nsIDeviceSensors {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensors) -> &Self {
        v
    }
}

impl nsIDeviceSensors {
    /// Cast this `nsIDeviceSensors` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDeviceSensorsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDeviceSensors {
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
impl<T: nsISupportsCoerce> nsIDeviceSensorsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeviceSensors) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDeviceSensors
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDeviceSensorsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub HasWindowListener: unsafe extern "system" fn (this: *const nsIDeviceSensors, aType: u32, aWindow: *const nsIDOMWindow, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub AddWindowListener: unsafe extern "system" fn (this: *const nsIDeviceSensors, aType: u32, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult,

    /* [noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow); */
    pub RemoveWindowListener: unsafe extern "system" fn (this: *const nsIDeviceSensors, aType: u32, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult,

    /* [noscript] void removeWindowAsListener (in nsIDOMWindow aWindow); */
    pub RemoveWindowAsListener: unsafe extern "system" fn (this: *const nsIDeviceSensors, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDeviceSensors {

    /// ```text
    /// /**
    ///    * Returns true if the given window has any listeners of the given type
    ///    */
    /// ```
    ///

    /// `bool hasWindowListener (in unsigned long aType, in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn HasWindowListener(&self, aType: u32, aWindow: *const nsIDOMWindow, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasWindowListener)(self, aType, aWindow, _retval)
    }



    /// `[noscript] void addWindowListener (in unsigned long aType, in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn AddWindowListener(&self, aType: u32, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).AddWindowListener)(self, aType, aWindow)
    }



    /// `[noscript] void removeWindowListener (in unsigned long aType, in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn RemoveWindowListener(&self, aType: u32, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWindowListener)(self, aType, aWindow)
    }



    /// `[noscript] void removeWindowAsListener (in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn RemoveWindowAsListener(&self, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWindowAsListener)(self, aWindow)
    }


}


