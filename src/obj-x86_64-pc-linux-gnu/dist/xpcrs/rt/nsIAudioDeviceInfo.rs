//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIAudioDeviceInfo.idl
//


/// `interface nsIAudioDeviceInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAudioDeviceInfo {
    vtable: *const nsIAudioDeviceInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAudioDeviceInfo.
unsafe impl XpCom for nsIAudioDeviceInfo {
    const IID: nsIID = nsID(0xfeb979a8, 0xf8cc, 0x4522,
        [0x9d, 0xff, 0x6c, 0x05, 0x5c, 0xa5, 0x07, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAudioDeviceInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAudioDeviceInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAudioDeviceInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIAudioDeviceInfo`.
    fn coerce_from(v: &nsIAudioDeviceInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAudioDeviceInfoCoerce for nsIAudioDeviceInfo {
    #[inline]
    fn coerce_from(v: &nsIAudioDeviceInfo) -> &Self {
        v
    }
}

impl nsIAudioDeviceInfo {
    /// Cast this `nsIAudioDeviceInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAudioDeviceInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAudioDeviceInfo {
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
impl<T: nsISupportsCoerce> nsIAudioDeviceInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioDeviceInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAudioDeviceInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAudioDeviceInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString groupId; */
    pub GetGroupId: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString vendor; */
    pub GetVendor: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aVendor: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aType: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned short state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aState: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned short preferred; */
    pub GetPreferred: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aPreferred: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned short supportedFormat; */
    pub GetSupportedFormat: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aSupportedFormat: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned short defaultFormat; */
    pub GetDefaultFormat: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aDefaultFormat: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maxChannels; */
    pub GetMaxChannels: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aMaxChannels: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long defaultRate; */
    pub GetDefaultRate: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aDefaultRate: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maxRate; */
    pub GetMaxRate: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aMaxRate: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long minRate; */
    pub GetMinRate: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aMinRate: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maxLatency; */
    pub GetMaxLatency: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aMaxLatency: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long minLatency; */
    pub GetMinLatency: unsafe extern "system" fn (this: *const nsIAudioDeviceInfo, aMinLatency: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAudioDeviceInfo {

    pub const TYPE_UNKNOWN: i64 = 0;


    pub const TYPE_INPUT: i64 = 1;


    pub const TYPE_OUTPUT: i64 = 2;


    pub const STATE_DISABLED: i64 = 0;


    pub const STATE_UNPLUGGED: i64 = 1;


    pub const STATE_ENABLED: i64 = 2;


    pub const PREF_NONE: i64 = 0;


    pub const PREF_MULTIMEDIA: i64 = 1;


    pub const PREF_VOICE: i64 = 2;


    pub const PREF_NOTIFICATION: i64 = 4;


    pub const PREF_ALL: i64 = 15;


    pub const FMT_S16LE: i64 = 16;


    pub const FMT_S16BE: i64 = 32;


    pub const FMT_F32LE: i64 = 4096;


    pub const FMT_F32BE: i64 = 8192;


    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `readonly attribute AString groupId;`
    #[inline]
    pub unsafe fn GetGroupId(&self, aGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetGroupId)(self, aGroupId)
    }



    /// `readonly attribute AString vendor;`
    #[inline]
    pub unsafe fn GetVendor(&self, aVendor: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVendor)(self, aVendor)
    }



    /// `readonly attribute unsigned short type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute unsigned short state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }



    /// `readonly attribute unsigned short preferred;`
    #[inline]
    pub unsafe fn GetPreferred(&self, aPreferred: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetPreferred)(self, aPreferred)
    }



    /// `readonly attribute unsigned short supportedFormat;`
    #[inline]
    pub unsafe fn GetSupportedFormat(&self, aSupportedFormat: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedFormat)(self, aSupportedFormat)
    }



    /// `readonly attribute unsigned short defaultFormat;`
    #[inline]
    pub unsafe fn GetDefaultFormat(&self, aDefaultFormat: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultFormat)(self, aDefaultFormat)
    }



    /// `readonly attribute unsigned long maxChannels;`
    #[inline]
    pub unsafe fn GetMaxChannels(&self, aMaxChannels: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxChannels)(self, aMaxChannels)
    }



    /// `readonly attribute unsigned long defaultRate;`
    #[inline]
    pub unsafe fn GetDefaultRate(&self, aDefaultRate: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultRate)(self, aDefaultRate)
    }



    /// `readonly attribute unsigned long maxRate;`
    #[inline]
    pub unsafe fn GetMaxRate(&self, aMaxRate: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxRate)(self, aMaxRate)
    }



    /// `readonly attribute unsigned long minRate;`
    #[inline]
    pub unsafe fn GetMinRate(&self, aMinRate: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMinRate)(self, aMinRate)
    }



    /// `readonly attribute unsigned long maxLatency;`
    #[inline]
    pub unsafe fn GetMaxLatency(&self, aMaxLatency: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxLatency)(self, aMaxLatency)
    }



    /// `readonly attribute unsigned long minLatency;`
    #[inline]
    pub unsafe fn GetMinLatency(&self, aMinLatency: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMinLatency)(self, aMinLatency)
    }


}


