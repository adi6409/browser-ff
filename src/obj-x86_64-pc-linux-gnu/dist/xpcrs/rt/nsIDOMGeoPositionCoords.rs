//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionCoords.idl
//


/// `interface nsIDOMGeoPositionCoords : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMGeoPositionCoords {
    vtable: *const nsIDOMGeoPositionCoordsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMGeoPositionCoords.
unsafe impl XpCom for nsIDOMGeoPositionCoords {
    const IID: nsIID = nsID(0xb31702d0, 0x6dac, 0x4fa0,
        [0xb9, 0x3b, 0xf0, 0x43, 0xe7, 0x1c, 0x8f, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMGeoPositionCoords {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMGeoPositionCoords.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMGeoPositionCoordsCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMGeoPositionCoords`.
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMGeoPositionCoordsCoerce for nsIDOMGeoPositionCoords {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionCoords {
    /// Cast this `nsIDOMGeoPositionCoords` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionCoordsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMGeoPositionCoords {
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
impl<T: nsISupportsCoerce> nsIDOMGeoPositionCoordsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionCoords) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMGeoPositionCoords
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMGeoPositionCoordsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute double latitude; */
    pub GetLatitude: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aLatitude: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double longitude; */
    pub GetLongitude: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aLongitude: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double altitude; */
    pub GetAltitude: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aAltitude: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double accuracy; */
    pub GetAccuracy: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aAccuracy: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double altitudeAccuracy; */
    pub GetAltitudeAccuracy: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aAltitudeAccuracy: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double heading; */
    pub GetHeading: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aHeading: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double speed; */
    pub GetSpeed: unsafe extern "system" fn (this: *const nsIDOMGeoPositionCoords, aSpeed: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMGeoPositionCoords {


    /// `readonly attribute double latitude;`
    #[inline]
    pub unsafe fn GetLatitude(&self, aLatitude: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetLatitude)(self, aLatitude)
    }



    /// `readonly attribute double longitude;`
    #[inline]
    pub unsafe fn GetLongitude(&self, aLongitude: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetLongitude)(self, aLongitude)
    }



    /// `readonly attribute double altitude;`
    #[inline]
    pub unsafe fn GetAltitude(&self, aAltitude: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetAltitude)(self, aAltitude)
    }



    /// `readonly attribute double accuracy;`
    #[inline]
    pub unsafe fn GetAccuracy(&self, aAccuracy: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetAccuracy)(self, aAccuracy)
    }



    /// `readonly attribute double altitudeAccuracy;`
    #[inline]
    pub unsafe fn GetAltitudeAccuracy(&self, aAltitudeAccuracy: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetAltitudeAccuracy)(self, aAltitudeAccuracy)
    }



    /// `readonly attribute double heading;`
    #[inline]
    pub unsafe fn GetHeading(&self, aHeading: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetHeading)(self, aHeading)
    }



    /// `readonly attribute double speed;`
    #[inline]
    pub unsafe fn GetSpeed(&self, aSpeed: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetSpeed)(self, aSpeed)
    }


}


