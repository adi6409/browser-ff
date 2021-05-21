//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMWindowUtils.idl
//


/// `interface nsIDOMWindowUtils : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMWindowUtils {
    vtable: *const nsIDOMWindowUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMWindowUtils.
unsafe impl XpCom for nsIDOMWindowUtils {
    const IID: nsIID = nsID(0x4d6732ca, 0x9da7, 0x4176,
        [0xb8, 0xa1, 0x8d, 0xde, 0x15, 0xcd, 0x0b, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMWindowUtils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMWindowUtils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMWindowUtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMWindowUtils`.
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMWindowUtilsCoerce for nsIDOMWindowUtils {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self {
        v
    }
}

impl nsIDOMWindowUtils {
    /// Cast this `nsIDOMWindowUtils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMWindowUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMWindowUtils {
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
impl<T: nsISupportsCoerce> nsIDOMWindowUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMWindowUtils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMWindowUtils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMWindowUtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned short imageAnimationMode; */
    pub GetImageAnimationMode: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aImageAnimationMode: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short imageAnimationMode; */
    pub SetImageAnimationMode: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aImageAnimationMode: u16) -> ::nserror::nsresult,

    /* readonly attribute boolean docCharsetIsForced; */
    pub GetDocCharsetIsForced: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDocCharsetIsForced: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute float physicalMillimeterInCSSPixels; */
    pub GetPhysicalMillimeterInCSSPixels: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPhysicalMillimeterInCSSPixels: *mut libc::c_float) -> ::nserror::nsresult,

    /* AString getDocumentMetadata (in AString aName); */
    pub GetDocumentMetadata: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void updateLayerTree (); */
    pub UpdateLayerTree: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long lastTransactionId; */
    pub GetLastTransactionId: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aLastTransactionId: *mut u64) -> ::nserror::nsresult,

    /* void getViewportInfo (in uint32_t aDisplayWidth, in uint32_t aDisplayHeight, out double aDefaultZoom, out boolean aAllowZoom, out double aMinZoom, out double aMaxZoom, out uint32_t aWidth, out uint32_t aHeight, out boolean aAutoSize); */
    pub GetViewportInfo: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t, aDefaultZoom: *mut libc::c_double, aAllowZoom: *mut bool, aMinZoom: *mut libc::c_double, aMaxZoom: *mut libc::c_double, aWidth: *mut uint32_t, aHeight: *mut uint32_t, aAutoSize: *mut bool) -> ::nserror::nsresult,

    /* AString getViewportFitInfo (); */
    pub GetViewportFitInfo: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void getContentViewerSize (out uint32_t aDisplayWidth, out uint32_t aDisplayHeight); */
    pub GetContentViewerSize: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDisplayWidth: *mut uint32_t, aDisplayHeight: *mut uint32_t) -> ::nserror::nsresult,

    /* void setDisplayPortForElement (in float aXPx, in float aYPx, in float aWidthPx, in float aHeightPx, in Element aElement, in uint32_t aPriority); */
    pub SetDisplayPortForElement: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aXPx: libc::c_float, aYPx: libc::c_float, aWidthPx: libc::c_float, aHeightPx: libc::c_float, aElement: *const libc::c_void, aPriority: uint32_t) -> ::nserror::nsresult,

    /* void setDisplayPortMarginsForElement (in float aLeftMargin, in float aTopMargin, in float aRightMargin, in float aBottomMargin, in Element aElement, in uint32_t aPriority); */
    pub SetDisplayPortMarginsForElement: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aLeftMargin: libc::c_float, aTopMargin: libc::c_float, aRightMargin: libc::c_float, aBottomMargin: libc::c_float, aElement: *const libc::c_void, aPriority: uint32_t) -> ::nserror::nsresult,

    /* void setDisplayPortBaseForElement (in int32_t aX, in int32_t aY, in int32_t aWidth, in int32_t aHeight, in Element aElement); */
    pub SetDisplayPortBaseForElement: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: int32_t, aY: int32_t, aWidth: int32_t, aHeight: int32_t, aElement: *const libc::c_void) -> ::nserror::nsresult,

    /* void getScrollbarSizes (in Element aElement, out uint32_t aVerticalScrollbarWidth, out uint32_t aHorizontalScrollbarHeight); */
    pub GetScrollbarSizes: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aVerticalScrollbarWidth: *mut uint32_t, aHorizontalScrollbarHeight: *mut uint32_t) -> ::nserror::nsresult,

    /* void setResolutionAndScaleTo (in float aResolution); */
    pub SetResolutionAndScaleTo: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aResolution: libc::c_float) -> ::nserror::nsresult,

    /* float getResolution (); */
    pub GetResolution: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut libc::c_float) -> ::nserror::nsresult,

    /* void setRestoreResolution (in float aResolution, in uint32_t aDisplayWidth, in uint32_t aDisplayHeight); */
    pub SetRestoreResolution: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aResolution: libc::c_float, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t) -> ::nserror::nsresult,

    /* attribute boolean isFirstPaint; */
    pub GetIsFirstPaint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsFirstPaint: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isFirstPaint; */
    pub SetIsFirstPaint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsFirstPaint: bool) -> ::nserror::nsresult,

    /* uint32_t getPresShellId (); */
    pub GetPresShellId: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* boolean isCORSSafelistedRequestHeader (in ACString name, in ACString value); */
    pub IsCORSSafelistedRequestHeader: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, name: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script,optional_argc] boolean sendMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub SendMouseEvent: *const ::libc::c_void,

    /* [can_run_script] boolean sendTouchEvent (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    pub SendTouchEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aType: *const ::nsstring::nsAString, aIdentifiers: *const thin_vec::ThinVec<uint32_t>, aXs: *const thin_vec::ThinVec<int32_t>, aYs: *const thin_vec::ThinVec<int32_t>, aRxs: *const thin_vec::ThinVec<uint32_t>, aRys: *const thin_vec::ThinVec<uint32_t>, aRotationAngles: *const thin_vec::ThinVec<libc::c_float>, aForces: *const thin_vec::ThinVec<libc::c_float>, aModifiers: i32, aIgnoreRootScrollFrame: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script,optional_argc] void sendMouseEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub SendMouseEventToWindow: *const ::libc::c_void,

    /* [can_run_script] boolean sendTouchEventToWindow (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
    pub SendTouchEventToWindow: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aType: *const ::nsstring::nsAString, aIdentifiers: *const thin_vec::ThinVec<uint32_t>, aXs: *const thin_vec::ThinVec<int32_t>, aYs: *const thin_vec::ThinVec<int32_t>, aRxs: *const thin_vec::ThinVec<uint32_t>, aRys: *const thin_vec::ThinVec<uint32_t>, aRotationAngles: *const thin_vec::ThinVec<libc::c_float>, aForces: *const thin_vec::ThinVec<libc::c_float>, aModifiers: i32, aIgnoreRootScrollFrame: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* void sendWheelEvent (in float aX, in float aY, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aDeltaMode, in long aModifiers, in long aLineOrPageDeltaX, in long aLineOrPageDeltaY, in unsigned long aOptions); */
    pub SendWheelEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aDeltaMode: u32, aModifiers: i32, aLineOrPageDeltaX: i32, aLineOrPageDeltaY: i32, aOptions: u32) -> ::nserror::nsresult,

    /* void sendNativeKeyEvent (in long aNativeKeyboardLayout, in long aNativeKeyCode, in long aModifierFlags, in AString aCharacters, in AString aUnmodifiedCharacters, [optional] in nsIObserver aObserver); */
    pub SendNativeKeyEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aNativeKeyboardLayout: i32, aNativeKeyCode: i32, aModifierFlags: i32, aCharacters: *const ::nsstring::nsAString, aUnmodifiedCharacters: *const ::nsstring::nsAString, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void sendNativeMouseEvent (in long aScreenX, in long aScreenY, in long aNativeMessage, in long aModifierFlags, in Element aElement, [optional] in nsIObserver aObserver); */
    pub SendNativeMouseEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenX: i32, aScreenY: i32, aNativeMessage: i32, aModifierFlags: i32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void sendNativeMouseMove (in long aScreenX, in long aScreenY, in Element aElement, [optional] in nsIObserver aObserver); */
    pub SendNativeMouseMove: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenX: i32, aScreenY: i32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void suppressAnimation (in boolean aSuppress); */
    pub SuppressAnimation: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSuppress: bool) -> ::nserror::nsresult,

    /* void sendNativeMouseScrollEvent (in long aScreenX, in long aScreenY, in unsigned long aNativeMessage, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aModifierFlags, in unsigned long aAdditionalFlags, in Element aElement, [optional] in nsIObserver aObserver); */
    pub SendNativeMouseScrollEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenX: i32, aScreenY: i32, aNativeMessage: u32, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aModifierFlags: u32, aAdditionalFlags: u32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void sendNativeTouchPoint (in unsigned long aPointerId, in unsigned long aTouchState, in long aScreenX, in long aScreenY, in double aPressure, in unsigned long aOrientation, [optional] in nsIObserver aObserver); */
    pub SendNativeTouchPoint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPointerId: u32, aTouchState: u32, aScreenX: i32, aScreenY: i32, aPressure: libc::c_double, aOrientation: u32, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void sendNativeTouchpadPinch (in unsigned long aEventPhase, in float aScale, in long aScreenX, in long aScreenY, in long aModifierFlags); */
    pub SendNativeTouchpadPinch: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aEventPhase: u32, aScale: libc::c_float, aScreenX: i32, aScreenY: i32, aModifierFlags: i32) -> ::nserror::nsresult,

    /* void sendNativeTouchTap (in long aScreenX, in long aScreenY, in boolean aLongTap, [optional] in nsIObserver aObserver); */
    pub SendNativeTouchTap: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenX: i32, aScreenY: i32, aLongTap: bool, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void clearNativeTouchSequence ([optional] in nsIObserver aObserver); */
    pub ClearNativeTouchSequence: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void clearSharedStyleSheetCache (); */
    pub ClearSharedStyleSheetCache: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute unsigned long parsedStyleSheets; */
    pub GetParsedStyleSheets: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aParsedStyleSheets: *mut u32) -> ::nserror::nsresult,

    /* void activateNativeMenuItemAt (in AString indexString); */
    pub ActivateNativeMenuItemAt: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, indexString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void forceUpdateNativeMenuAt (in AString indexString); */
    pub ForceUpdateNativeMenuAt: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, indexString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString GetSelectionAsPlaintext (); */
    pub GetSelectionAsPlaintext: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void garbageCollect ([optional] in nsICycleCollectorListener aListener); */
    pub GarbageCollect: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult,

    /* void cycleCollect ([optional] in nsICycleCollectorListener aListener); */
    pub CycleCollect: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult,

    /* void runNextCollectorTimer (); */
    pub RunNextCollectorTimer: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void sendSimpleGestureEvent (in AString aType, in float aX, in float aY, in unsigned long aDirection, in double aDelta, in long aModifiers, [optional] in unsigned long aClickCount); */
    pub SendSimpleGestureEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aType: *const ::nsstring::nsAString, aX: libc::c_float, aY: libc::c_float, aDirection: u32, aDelta: libc::c_double, aModifiers: i32, aClickCount: u32) -> ::nserror::nsresult,

    /* Element elementFromPoint (in float aX, in float aY, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
    pub ElementFromPoint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* NodeList nodesFromRect (in float aX, in float aY, in float aTopSize, in float aRightSize, in float aBottomSize, in float aLeftSize, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout, in boolean aOnlyVisible, [optional] in float aTransparencyThreshold); */
    pub NodesFromRect: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aTopSize: libc::c_float, aRightSize: libc::c_float, aBottomSize: libc::c_float, aLeftSize: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, aOnlyVisible: bool, aTransparencyThreshold: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* nsITranslationNodeList getTranslationNodes (in Node aRoot); */
    pub GetTranslationNodes: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aRoot: *const libc::c_void, _retval: *mut*const nsITranslationNodeList) -> ::nserror::nsresult,

    /* uint32_t compareCanvases (in nsISupports aCanvas1, in nsISupports aCanvas2, out unsigned long aMaxDifference); */
    pub CompareCanvases: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aCanvas1: *const nsISupports, aCanvas2: *const nsISupports, aMaxDifference: *mut u32, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean isMozAfterPaintPending; */
    pub GetIsMozAfterPaintPending: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsMozAfterPaintPending: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isInputTaskManagerSuspended; */
    pub GetIsInputTaskManagerSuspended: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsInputTaskManagerSuspended: *mut bool) -> ::nserror::nsresult,

    /* void suppressEventHandling (in boolean aSuppress); */
    pub SuppressEventHandling: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSuppress: bool) -> ::nserror::nsresult,

    /* void disableNonTestMouseEvents (in boolean aDisable); */
    pub DisableNonTestMouseEvents: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDisable: bool) -> ::nserror::nsresult,

    /* void getScrollXY (in boolean aFlushLayout, out long aScrollX, out long aScrollY); */
    pub GetScrollXY: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aScrollX: *mut i32, aScrollY: *mut i32) -> ::nserror::nsresult,

    /* void getScrollXYFloat (in boolean aFlushLayout, out float aScrollX, out float aScrollY); */
    pub GetScrollXYFloat: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float) -> ::nserror::nsresult,

    /* void getScrollbarSize (in boolean aFlushLayout, out long aWidth, out long aHeight); */
    pub GetScrollbarSize: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFlushLayout: bool, aWidth: *mut i32, aHeight: *mut i32) -> ::nserror::nsresult,

    /* DOMRect getBoundsWithoutFlushing (in Element aElement); */
    pub GetBoundsWithoutFlushing: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void scrollToVisual (in float aOffsetX, in float aOffsetY, in long aUpdateType, in long aScrollMode); */
    pub ScrollToVisual: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aOffsetX: libc::c_float, aOffsetY: libc::c_float, aUpdateType: i32, aScrollMode: i32) -> ::nserror::nsresult,

    /* void getVisualViewportOffsetRelativeToLayoutViewport (out float aOffsetX, out float aOffsetY); */
    pub GetVisualViewportOffsetRelativeToLayoutViewport: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aOffsetX: *mut libc::c_float, aOffsetY: *mut libc::c_float) -> ::nserror::nsresult,

    /* void getVisualViewportOffset (out long aOffsetX, out long aOffsetY); */
    pub GetVisualViewportOffset: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aOffsetX: *mut i32, aOffsetY: *mut i32) -> ::nserror::nsresult,

    /* DOMRect transformRectLayoutToVisual (in float aX, in float aY, in float aWidth, in float aHeight); */
    pub TransformRectLayoutToVisual: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aWidth: libc::c_float, aHeight: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* DOMRect toScreenRect (in float aX, in float aY, in float aWidth, in float aHeight); */
    pub ToScreenRect: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aWidth: libc::c_float, aHeight: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void setDynamicToolbarMaxHeight (in uint32_t aHeightInScreen); */
    pub SetDynamicToolbarMaxHeight: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aHeightInScreen: uint32_t) -> ::nserror::nsresult,

    /* bool needsFlush (in long aFlushtype); */
    pub NeedsFlush: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFlushtype: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* void flushLayoutWithoutThrottledAnimations (); */
    pub FlushLayoutWithoutThrottledAnimations: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* DOMRect getRootBounds (); */
    pub GetRootBounds: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean IMEIsOpen; */
    pub GetIMEIsOpen: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIMEIsOpen: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long IMEStatus; */
    pub GetIMEStatus: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIMEStatus: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute float screenPixelsPerCSSPixel; */
    pub GetScreenPixelsPerCSSPixel: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenPixelsPerCSSPixel: *mut libc::c_float) -> ::nserror::nsresult,

    /* readonly attribute float screenPixelsPerCSSPixelNoOverride; */
    pub GetScreenPixelsPerCSSPixelNoOverride: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aScreenPixelsPerCSSPixelNoOverride: *mut libc::c_float) -> ::nserror::nsresult,

    /* readonly attribute float fullZoom; */
    pub GetFullZoom: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFullZoom: *mut libc::c_float) -> ::nserror::nsresult,

    /* [can_run_script] boolean dispatchDOMEventViaPresShellForTesting (in Node aTarget, in Event aEvent); */
    pub DispatchDOMEventViaPresShellForTesting: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aTarget: *const libc::c_void, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean dispatchEventToChromeOnly (in EventTarget aTarget, in Event aEvent); */
    pub DispatchEventToChromeOnly: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aTarget: *const libc::c_void, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] string getClassName (in jsval aObject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetClassName: *const ::libc::c_void,

    /* void sendContentCommandEvent (in AString aType, [optional] in nsITransferable aTransferable); */
    pub SendContentCommandEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aType: *const ::nsstring::nsAString, aTransferable: *const nsITransferable) -> ::nserror::nsresult,

    /* nsIQueryContentEventResult sendQueryContentEvent (in unsigned long aType, in long long aOffset, in unsigned long aLength, in long aX, in long aY, [optional] in unsigned long aAdditionalFlags); */
    pub SendQueryContentEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aType: u32, aOffset: i64, aLength: u32, aX: i32, aY: i32, aAdditionalFlags: u32, _retval: *mut*const nsIQueryContentEventResult) -> ::nserror::nsresult,

    /* void remoteFrameFullscreenChanged (in Element aFrameElement); */
    pub RemoteFrameFullscreenChanged: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFrameElement: *const libc::c_void) -> ::nserror::nsresult,

    /* void remoteFrameFullscreenReverted (); */
    pub RemoteFrameFullscreenReverted: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* boolean handleFullscreenRequests (); */
    pub HandleFullscreenRequests: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> ::nserror::nsresult,

    /* void exitFullscreen (); */
    pub ExitFullscreen: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* boolean sendSelectionSetEvent (in unsigned long aOffset, in unsigned long aLength, [optional] in unsigned long aAdditionalFlags); */
    pub SendSelectionSetEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aOffset: u32, aLength: u32, aAdditionalFlags: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] boolean selectAtPoint (in float aX, in float aY, in unsigned long aSelectBehavior); */
    pub SelectAtPoint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aX: libc::c_float, aY: libc::c_float, aSelectBehavior: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getVisitedDependentComputedStyle (in Element aElement, in AString aPseudoElement, in AString aPropertyName); */
    pub GetVisitedDependentComputedStyle: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, aPropertyName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long deprecatedOuterWindowID; */
    pub GetDeprecatedOuterWindowID: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDeprecatedOuterWindowID: *mut u64) -> ::nserror::nsresult,

    /* void enterModalState (); */
    pub EnterModalState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void leaveModalState (); */
    pub LeaveModalState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* boolean isInModalState (); */
    pub IsInModalState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean desktopModeViewport; */
    pub GetDesktopModeViewport: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDesktopModeViewport: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean desktopModeViewport; */
    pub SetDesktopModeViewport: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDesktopModeViewport: bool) -> ::nserror::nsresult,

    /* void suspendTimeouts (); */
    pub SuspendTimeouts: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void resumeTimeouts (); */
    pub ResumeTimeouts: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute AString layerManagerType; */
    pub GetLayerManagerType: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aLayerManagerType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean layerManagerRemote; */
    pub GetLayerManagerRemote: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aLayerManagerRemote: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean usingAdvancedLayers; */
    pub GetUsingAdvancedLayers: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aUsingAdvancedLayers: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isWebRenderRequested; */
    pub GetIsWebRenderRequested: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsWebRenderRequested: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString currentAudioBackend; */
    pub GetCurrentAudioBackend: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aCurrentAudioBackend: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long currentMaxAudioChannels; */
    pub GetCurrentMaxAudioChannels: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aCurrentMaxAudioChannels: *mut u32) -> ::nserror::nsresult,

    /* Promise defaultDevicesRoundTripLatency (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub DefaultDevicesRoundTripLatency: *const ::libc::c_void,

    /* readonly attribute unsigned long currentPreferredSampleRate; */
    pub GetCurrentPreferredSampleRate: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aCurrentPreferredSampleRate: *mut u32) -> ::nserror::nsresult,

    /* nsIArray audioDevices (in unsigned short aSide); */
    pub AudioDevices: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSide: u16, _retval: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void startFrameTimeRecording ([retval] out unsigned long startIndex); */
    pub StartFrameTimeRecording: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, startIndex: *mut u32) -> ::nserror::nsresult,

    /* Array<float> stopFrameTimeRecording (in unsigned long startIndex); */
    pub StopFrameTimeRecording: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, startIndex: u32, _retval: *mut thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult,

    /* readonly attribute float displayDPI; */
    pub GetDisplayDPI: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aDisplayDPI: *mut libc::c_float) -> ::nserror::nsresult,

    /* void advanceTimeAndRefresh (in long long aMilliseconds); */
    pub AdvanceTimeAndRefresh: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aMilliseconds: i64) -> ::nserror::nsresult,

    /* void restoreNormalRefresh (); */
    pub RestoreNormalRefresh: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute bool isTestControllingRefreshes; */
    pub GetIsTestControllingRefreshes: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsTestControllingRefreshes: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool asyncPanZoomEnabled; */
    pub GetAsyncPanZoomEnabled: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aAsyncPanZoomEnabled: *mut bool) -> ::nserror::nsresult,

    /* void setAsyncScrollOffset (in Element aElement, in float aX, in float aY); */
    pub SetAsyncScrollOffset: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aX: libc::c_float, aY: libc::c_float) -> ::nserror::nsresult,

    /* void setAsyncZoom (in Element aRootElement, in float aValue); */
    pub SetAsyncZoom: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aRootElement: *const libc::c_void, aValue: libc::c_float) -> ::nserror::nsresult,

    /* bool flushApzRepaints (); */
    pub FlushApzRepaints: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> ::nserror::nsresult,

    /* void disableApzForElement (in Element aElement); */
    pub DisableApzForElement: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void zoomToFocusedInput (); */
    pub ZoomToFocusedInput: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* double computeAnimationDistance (in Element element, in AString property, in AString value1, in AString value2); */
    pub ComputeAnimationDistance: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, element: *const libc::c_void, property: *const ::nsstring::nsAString, value1: *const ::nsstring::nsAString, value2: *const ::nsstring::nsAString, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* AString getUnanimatedComputedStyle (in Element aElement, in AString aPseudoElement, in AString aProperty, in long aFlushType); */
    pub GetUnanimatedComputedStyle: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, aProperty: *const ::nsstring::nsAString, aFlushType: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString focusedInputType; */
    pub GetFocusedInputType: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFocusedInputType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString focusedActionHint; */
    pub GetFocusedActionHint: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFocusedActionHint: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString focusedInputMode; */
    pub GetFocusedInputMode: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFocusedInputMode: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString focusedAutocapitalize; */
    pub GetFocusedAutocapitalize: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFocusedAutocapitalize: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsViewID getViewId (in Element aElement); */
    pub GetViewId: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, _retval: *mut nsViewID) -> ::nserror::nsresult,

    /* boolean leafLayersPartitionWindow (); */
    pub LeafLayersPartitionWindow: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean checkAndClearPaintedState (in Element aElement); */
    pub CheckAndClearPaintedState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean checkAndClearDisplayListState (in Element aElement); */
    pub CheckAndClearDisplayListState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isPartOfOpaqueLayer (in Element aElement); */
    pub IsPartOfOpaqueLayer: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* unsigned long numberOfAssignedPaintedLayers (in Array<Element> aElements); */
    pub NumberOfAssignedPaintedLayers: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElements: *const thin_vec::ThinVec<*const libc::c_void>, _retval: *mut u32) -> ::nserror::nsresult,

    /* [implicit_jscontext] long long getFileId (in jsval aFile); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetFileId: *const ::libc::c_void,

    /* [implicit_jscontext] AString getFilePath (in jsval aFile); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetFilePath: *const ::libc::c_void,

    /* [implicit_jscontext] boolean getFileReferences (in AString aDatabaseName, in long long aId, [optional] in jsval aOptions, [optional] out long aRefCnt, [optional] out long aDBRefCnt); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetFileReferences: *const ::libc::c_void,

    /* void flushPendingFileDeletions (); */
    pub FlushPendingFileDeletions: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* [implicit_jscontext] void startPCCountProfiling (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub StartPCCountProfiling: *const ::libc::c_void,

    /* [implicit_jscontext] void stopPCCountProfiling (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub StopPCCountProfiling: *const ::libc::c_void,

    /* [implicit_jscontext] void purgePCCounts (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub PurgePCCounts: *const ::libc::c_void,

    /* [implicit_jscontext] long getPCCountScriptCount (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetPCCountScriptCount: *const ::libc::c_void,

    /* [implicit_jscontext] AString getPCCountScriptSummary (in long script); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetPCCountScriptSummary: *const ::libc::c_void,

    /* [implicit_jscontext] AString getPCCountScriptContents (in long script); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetPCCountScriptContents: *const ::libc::c_void,

    /* readonly attribute boolean paintingSuppressed; */
    pub GetPaintingSuppressed: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPaintingSuppressed: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval plugins; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetPlugins: *const ::libc::c_void,

    /* void setVisualViewportSize (in float aWidth, in float aHeight); */
    pub SetVisualViewportSize: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aWidth: libc::c_float, aHeight: libc::c_float) -> ::nserror::nsresult,

    /* void disableDialogs (); */
    pub DisableDialogs: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void enableDialogs (); */
    pub EnableDialogs: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* bool areDialogsEnabled (); */
    pub AreDialogsEnabled: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut bool) -> ::nserror::nsresult,

    /* void loadSheet (in nsIURI sheetURI, in unsigned long type); */
    pub LoadSheet: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult,

    /* void loadSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    pub LoadSheetUsingURIString: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, sheetURI: *const ::nsstring::nsACString, type_: u32) -> ::nserror::nsresult,

    /* void addSheet (in nsIPreloadedStyleSheet sheet, in unsigned long type); */
    pub AddSheet: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, sheet: *const nsIPreloadedStyleSheet, type_: u32) -> ::nserror::nsresult,

    /* void removeSheet (in nsIURI sheetURI, in unsigned long type); */
    pub RemoveSheet: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult,

    /* void removeSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
    pub RemoveSheetUsingURIString: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, sheetURI: *const ::nsstring::nsACString, type_: u32) -> ::nserror::nsresult,

    /* readonly attribute boolean isHandlingUserInput; */
    pub GetIsHandlingUserInput: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsHandlingUserInput: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute double millisSinceLastUserInput; */
    pub GetMillisSinceLastUserInput: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aMillisSinceLastUserInput: *mut libc::c_double) -> ::nserror::nsresult,

    /* void allowScriptsToClose (); */
    pub AllowScriptsToClose: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute boolean isParentWindowMainWidgetVisible; */
    pub GetIsParentWindowMainWidgetVisible: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aIsParentWindowMainWidgetVisible: *mut bool) -> ::nserror::nsresult,

    /* boolean isNodeDisabledForEvents (in Node aNode); */
    pub IsNodeDisabledForEvents: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aNode: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean paintFlashing; */
    pub GetPaintFlashing: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPaintFlashing: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean paintFlashing; */
    pub SetPaintFlashing: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPaintFlashing: bool) -> ::nserror::nsresult,

    /* AString getOMTAStyle (in Element aElement, in AString aProperty, [optional] in AString aPseudoElement); */
    pub GetOMTAStyle: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aProperty: *const ::nsstring::nsAString, aPseudoElement: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getOMTCTransform (in Element aElement, [optional] in AString aPseudoElement); */
    pub GetOMTCTransform: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool isAnimationInPendingTracker (in Animation aAnimation); */
    pub IsAnimationInPendingTracker: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aAnimation: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIJSRAIIHelper setHandlingUserInput (in boolean aHandlingInput); */
    pub SetHandlingUserInput: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aHandlingInput: bool, _retval: *mut*const nsIJSRAIIHelper) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getContentAPZTestData (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetContentAPZTestData: *const ::libc::c_void,

    /* [implicit_jscontext] jsval getCompositorAPZTestData (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetCompositorAPZTestData: *const ::libc::c_void,

    /* void postRestyleSelfEvent (in Element aElement); */
    pub PostRestyleSelfEvent: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aElement: *const libc::c_void) -> ::nserror::nsresult,

    /* void xpconnectArgument (in nsISupports aObj); */
    pub XpconnectArgument: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aObj: *const nsISupports) -> ::nserror::nsresult,

    /* void askPermission (in nsIContentPermissionRequest aRequest); */
    pub AskPermission: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aRequest: *const nsIContentPermissionRequest) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long restyleGeneration; */
    pub GetRestyleGeneration: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aRestyleGeneration: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long framesConstructed; */
    pub GetFramesConstructed: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFramesConstructed: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long framesReflowed; */
    pub GetFramesReflowed: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aFramesReflowed: *mut u64) -> ::nserror::nsresult,

    /* void setChromeMargin (in int32_t aTop, in int32_t aRight, in int32_t aBottom, in int32_t aLeft); */
    pub SetChromeMargin: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aTop: int32_t, aRight: int32_t, aBottom: int32_t, aLeft: int32_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getFrameUniformityTestData (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetFrameUniformityTestData: *const ::libc::c_void,

    /* void enterChaosMode (); */
    pub EnterChaosMode: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void leaveChaosMode (); */
    pub LeaveChaosMode: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void triggerDeviceReset (); */
    pub TriggerDeviceReset: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* bool hasRuleProcessorUsedByMultipleStyleSets (in unsigned long aSheetType); */
    pub HasRuleProcessorUsedByMultipleStyleSets: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSheetType: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* void respectDisplayPortSuppression (in boolean aEnabled); */
    pub RespectDisplayPortSuppression: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aEnabled: bool) -> ::nserror::nsresult,

    /* void forceReflowInterrupt (); */
    pub ForceReflowInterrupt: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void terminateGPUProcess (); */
    pub TerminateGPUProcess: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* readonly attribute int32_t gpuProcessPid; */
    pub GetGpuProcessPid: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aGpuProcessPid: *mut int32_t) -> ::nserror::nsresult,

    /* void addManuallyManagedState (in Element element, in AString state); */
    pub AddManuallyManagedState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, element: *const libc::c_void, state: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeManuallyManagedState (in Element element, in AString state); */
    pub RemoveManuallyManagedState: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, element: *const libc::c_void, state: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* int64_t getStorageUsage (in Storage aStorage); */
    pub GetStorageUsage: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aStorage: *const libc::c_void, _retval: *mut int64_t) -> ::nserror::nsresult,

    /* long getDirectionFromText (in AString aString); */
    pub GetDirectionFromText: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aString: *const ::nsstring::nsAString, _retval: *mut i32) -> ::nserror::nsresult,

    /* void ensureDirtyRootFrame (); */
    pub EnsureDirtyRootFrame: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void wrCapture (); */
    pub WrCapture: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* void wrToggleCaptureSequence (); */
    pub WrToggleCaptureSequence: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* Promise setCompositionRecording (in boolean aValue); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SetCompositionRecording: *const ::libc::c_void,

    /* Promise startCompositionRecording (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub StartCompositionRecording: *const ::libc::c_void,

    /* Promise stopCompositionRecording (in boolean aWriteToDisk); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub StopCompositionRecording: *const ::libc::c_void,

    /* bool isCssPropertyRecordedInUseCounter (in ACString aProperty); */
    pub IsCssPropertyRecordedInUseCounter: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aProperty: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void resetMobileViewportManager (); */
    pub ResetMobileViewportManager: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* attribute ACString systemFont; */
    pub GetSystemFont: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSystemFont: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString systemFont; */
    pub SetSystemFont: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aSystemFont: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long paintCount; */
    pub GetPaintCount: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aPaintCount: *mut u64) -> ::nserror::nsresult,

    /* void syncFlushCompositor (); */
    pub SyncFlushCompositor: unsafe extern "system" fn (this: *const nsIDOMWindowUtils) -> ::nserror::nsresult,

    /* unsigned long long getLayersId (); */
    pub GetLayersId: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, _retval: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute bool usesOverlayScrollbars; */
    pub GetUsesOverlayScrollbars: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aUsesOverlayScrollbars: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool effectivelyThrottlesFrameRequests; */
    pub GetEffectivelyThrottlesFrameRequests: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aEffectivelyThrottlesFrameRequests: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString webrtcRawDeviceId; */
    pub GetWebrtcRawDeviceId: unsafe extern "system" fn (this: *const nsIDOMWindowUtils, aWebrtcRawDeviceId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMWindowUtils {
    /// ```text
    /// /**
    ///    * Following modifiers are for sent*Event() except sendNative*Event().
    ///    * NOTE: MODIFIER_ALT, MODIFIER_CONTROL, MODIFIER_SHIFT and MODIFIER_META
    ///    *       are must be same values as Event_Binding::*_MASK for backward
    ///    *       compatibility.
    ///    */
    /// ```
    ///

    pub const MODIFIER_ALT: i64 = 1;


    pub const MODIFIER_CONTROL: i64 = 2;


    pub const MODIFIER_SHIFT: i64 = 4;


    pub const MODIFIER_META: i64 = 8;


    pub const MODIFIER_ALTGRAPH: i64 = 16;


    pub const MODIFIER_CAPSLOCK: i64 = 32;


    pub const MODIFIER_FN: i64 = 64;


    pub const MODIFIER_FNLOCK: i64 = 128;


    pub const MODIFIER_NUMLOCK: i64 = 256;


    pub const MODIFIER_SCROLLLOCK: i64 = 512;


    pub const MODIFIER_SYMBOL: i64 = 1024;


    pub const MODIFIER_SYMBOLLOCK: i64 = 2048;


    pub const MODIFIER_OS: i64 = 4096;

    /// ```text
    /// /** Synthesize a wheel event for a window. The event types supported is only
    ///    *  wheel.
    ///    *
    ///    * Events are sent in coordinates offset by aX and aY from the window.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * @param aX                 x offset in CSS pixels
    ///    * @param aY                 y offset in CSS pixels
    ///    * @param aDeltaX            deltaX value.
    ///    * @param aDeltaY            deltaY value.
    ///    * @param aDeltaZ            deltaZ value.
    ///    * @param aDeltaMode         deltaMode value which must be one of the
    ///    *                           WheelEvent DOM_DELTA_* constants.
    ///    * @param aModifiers         modifiers pressed, using constants defined as
    ///    *                           MODIFIER_*
    ///    * @param aLineOrPageDeltaX  If you set this value non-zero for
    ///    *                           DOM_DELTA_PIXEL event, EventStateManager will
    ///    *                           dispatch NS_MOUSE_SCROLL event for horizontal
    ///    *                           scroll.
    ///    * @param aLineOrPageDeltaY  If you set this value non-zero for
    ///    *                           DOM_DELTA_PIXEL event, EventStateManager will
    ///    *                           dispatch NS_MOUSE_SCROLL event for vertical
    ///    *                           scroll.
    ///    * @param aOptions           Set following flags.
    ///    */
    /// ```
    ///

    pub const WHEEL_EVENT_CAUSED_BY_NO_LINE_OR_PAGE_DELTA_DEVICE: i64 = 1;


    pub const WHEEL_EVENT_CAUSED_BY_MOMENTUM: i64 = 2;


    pub const WHEEL_EVENT_CUSTOMIZED_BY_USER_PREFS: i64 = 4;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_ZERO: i64 = 16;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_POSITIVE: i64 = 32;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_NEGATIVE: i64 = 64;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_ZERO: i64 = 256;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_POSITIVE: i64 = 512;


    pub const WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_NEGATIVE: i64 = 1024;

    /// ```text
    /// /**
    ///    * The values for sendNativeMouseScrollEvent's aAdditionalFlags.
    ///    */
    /// /**
    ///    * If MOUSESCROLL_PREFER_WIDGET_AT_POINT is set, widget will dispatch
    ///    * the event to a widget which is under the cursor.  Otherwise, dispatch to
    ///    * a default target on the platform.  E.g., on Windows, it's focused window.
    ///    */
    /// ```
    ///

    pub const MOUSESCROLL_PREFER_WIDGET_AT_POINT: i64 = 1;

    /// ```text
    /// /**
    ///    * Interpret the scroll delta values as lines rather than pixels.
    ///    */
    /// ```
    ///

    pub const MOUSESCROLL_SCROLL_LINES: i64 = 2;

    /// ```text
    /// /**
    ///    * The platform specific values of aAdditionalFlags.  Must be over 0x00010000.
    ///    */
    /// /**
    ///    * If MOUSESCROLL_WIN_SCROLL_LPARAM_NOT_NULL is set and aNativeMessage is
    ///    * WM_VSCROLL or WM_HSCROLL, widget will set the window handle to the lParam
    ///    * instead of NULL.
    ///    */
    /// ```
    ///

    pub const MOUSESCROLL_WIN_SCROLL_LPARAM_NOT_NULL: i64 = 65536;

    /// ```text
    /// /**
    ///    * Touch states for sendNativeTouchPoint. These values match
    ///    * nsIWidget's TouchPointerState.
    ///    */
    /// ```
    ///

    pub const TOUCH_HOVER: i64 = 1;


    pub const TOUCH_CONTACT: i64 = 2;


    pub const TOUCH_REMOVE: i64 = 4;


    pub const TOUCH_CANCEL: i64 = 8;

    /// ```text
    /// /**
    ///    * Phase states for sendNativeTouchPadPinch.
    ///    */
    /// ```
    ///

    pub const PHASE_BEGIN: i64 = 0;


    pub const PHASE_UPDATE: i64 = 1;


    pub const PHASE_END: i64 = 2;

    /// ```text
    /// /**
    ///    * Scroll the visual viewport to the given coordinates, relative to the
    ///    * document origin.
    ///    * Only applicable to the window associated with the root content document.
    ///    * Note: this does not take effect right away. Rather, the visual scroll
    ///    *       request is sent to APZ with the next transaction, and will be
    ///    *       reflected in the main thread with the subsequent APZ repaint request.
    ///    * Please see the caveats mentioned at PresShell::ScrollToVisual(), and
    ///    * request APZ review if adding a new call to this.
    ///    */
    /// ```
    ///

    pub const UPDATE_TYPE_RESTORE: i64 = 0;


    pub const UPDATE_TYPE_MAIN_THREAD: i64 = 1;


    pub const SCROLL_MODE_INSTANT: i64 = 0;


    pub const SCROLL_MODE_SMOOTH: i64 = 1;


    pub const FLUSH_NONE: i64 = -1;


    pub const FLUSH_STYLE: i64 = 0;


    pub const FLUSH_LAYOUT: i64 = 1;


    pub const FLUSH_DISPLAY: i64 = 2;

    /// ```text
    /// /**
    ///    * WARNING: These values must be same as nsIWidget's values.
    ///    */
    /// /**
    ///    * DISABLED means users cannot use IME completely.
    ///    * Note that this state is *not* same as |ime-mode: disabled;|.
    ///    */
    /// ```
    ///

    pub const IME_STATUS_DISABLED: i64 = 0;

    /// ```text
    /// /**
    ///    * ENABLED means users can use all functions of IME. This state is same as
    ///    * |ime-mode: normal;|.
    ///    */
    /// ```
    ///

    pub const IME_STATUS_ENABLED: i64 = 1;

    /// ```text
    /// /**
    ///    * PASSWORD means users cannot use most functions of IME. But on GTK2,
    ///    * users can use "Simple IM" which only supports dead key inputting.
    ///    * The behavior is same as the behavior of the native password field.
    ///    * This state is same as |ime-mode: disabled;|.
    ///    */
    /// ```
    ///

    pub const IME_STATUS_PASSWORD: i64 = 2;

    /// ```text
    /// /**
    ///    * If sendQueryContentEvent()'s aAdditionalFlags argument is
    ///    * QUERY_CONTENT_FLAG_USE_XP_LINE_BREAK, plain text generated from content
    ///    * is created with "\n".
    ///    * Otherwise, platform dependent.  E.g., on Windows, "\r\n" is used.
    ///    * aOffset and aLength are offset and length in/of the plain text content.
    ///    * This flag also affects the result values such as offset, length and string.
    ///    */
    /// ```
    ///

    pub const QUERY_CONTENT_FLAG_USE_NATIVE_LINE_BREAK: i64 = 0;


    pub const QUERY_CONTENT_FLAG_USE_XP_LINE_BREAK: i64 = 1;

    /// ```text
    /// /**
    ///    * sendQueryContentEvent()'s aAdditionalFlags may have one of following
    ///    * flags when aType is QUERY_SELECTED_TEXT.  If one of them is set,
    ///    * the result is the first range of the selection type.  See also
    ///    * nsISelectionController::SELECTION_*.
    ///    */
    /// ```
    ///

    pub const QUERY_CONTENT_FLAG_SELECTION_SPELLCHECK: i64 = 2;


    pub const QUERY_CONTENT_FLAG_SELECTION_IME_RAWINPUT: i64 = 4;


    pub const QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDRAWTEXT: i64 = 8;


    pub const QUERY_CONTENT_FLAG_SELECTION_IME_CONVERTEDTEXT: i64 = 16;


    pub const QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDCONVERTEDTEXT: i64 = 32;


    pub const QUERY_CONTENT_FLAG_SELECTION_ACCESSIBILITY: i64 = 64;


    pub const QUERY_CONTENT_FLAG_SELECTION_FIND: i64 = 128;


    pub const QUERY_CONTENT_FLAG_SELECTION_URLSECONDARY: i64 = 256;


    pub const QUERY_CONTENT_FLAG_SELECTION_URLSTRIKEOUT: i64 = 512;

    /// ```text
    /// /**
    ///    * One of sendQueryContentEvent()'s aAdditionalFlags.  If this is specified,
    ///    * aOffset is relative to start of selection or composition.
    ///    * Note that this is supported only when QUERY_CONTENT_FLAG_USE_XP_LINE_BREAK
    ///    * is not specified for now.
    ///    */
    /// ```
    ///

    pub const QUERY_CONTENT_FLAG_OFFSET_RELATIVE_TO_INSERTION_POINT: i64 = 1024;

    /// ```text
    /// /**
    ///    * QUERY_SELECTED_TEXT queries the first selection range's information.
    ///    *
    ///    * @param aOffset   Not used.
    ///    * @param aLength   Not used.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    *
    ///    * @return offset, reversed and text properties of the result are available.
    ///    */
    /// ```
    ///

    pub const QUERY_SELECTED_TEXT: i64 = 3200;

    /// ```text
    /// /**
    ///    * QUERY_TEXT_CONTENT queries the text at the specified range.
    ///    *
    ///    * @param aOffset   The first character's offset.  0 is the first character.
    ///    * @param aLength   The length of getting text.  If the aLength is too long,
    ///    *                  the result text is shorter than this value.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    *
    ///    * @return text property of the result is available.
    ///    */
    /// ```
    ///

    pub const QUERY_TEXT_CONTENT: i64 = 3201;

    /// ```text
    /// /**
    ///    * QUERY_CARET_RECT queries the (collapsed) caret rect of the offset.
    ///    * If the actual caret is there at the specified offset, this returns the
    ///    * actual caret rect.  Otherwise, this guesses the caret rect from the
    ///    * metrics of the text.
    ///    *
    ///    * @param aOffset   The caret offset.  0 is the left side of the first
    ///    *                  caracter in LTR text.
    ///    * @param aLength   Not used.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    *
    ///    * @return left, top, width and height properties of the result are available.
    ///    *         The left and the top properties are offset in the client area of
    ///    *         the DOM window.
    ///    */
    /// ```
    ///

    pub const QUERY_CARET_RECT: i64 = 3203;

    /// ```text
    /// /**
    ///    * QUERY_TEXT_RECT queries the specified text's rect.
    ///    *
    ///    * @param aOffset   The first character's offset.  0 is the first character.
    ///    * @param aLength   The length of getting text.  If the aLength is too long,
    ///    *                  the extra length is ignored.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    *
    ///    * @return left, top, width and height properties of the result are available.
    ///    *         The left and the top properties are offset in the client area of
    ///    *         the DOM window.
    ///    */
    /// ```
    ///

    pub const QUERY_TEXT_RECT: i64 = 3204;

    /// ```text
    /// /**
    ///    * QUERY_TEXT_RECT queries the focused editor's rect.
    ///    *
    ///    * @param aOffset   Not used.
    ///    * @param aLength   Not used.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    *
    ///    * @return left, top, width and height properties of the result are available.
    ///    */
    /// ```
    ///

    pub const QUERY_EDITOR_RECT: i64 = 3205;

    /// ```text
    /// /**
    ///    * QUERY_CHARACTER_AT_POINT queries the character information at the
    ///    * specified point.  The point is offset in the window.
    ///    * NOTE: If there are some panels at the point, this method send the query
    ///    * event to the panel's widget automatically.
    ///    *
    ///    * @param aOffset   Not used.
    ///    * @param aLength   Not used.
    ///    * @param aX        X offset in the widget.
    ///    * @param aY        Y offset in the widget.
    ///    *
    ///    * @return offset, notFound, left, top, width and height properties of the
    ///    *         result are available.
    ///    */
    /// ```
    ///

    pub const QUERY_CHARACTER_AT_POINT: i64 = 3208;

    /// ```text
    /// /**
    ///    * QUERY_TEXT_RECT_ARRAY queries the rects per character
    ///    *
    ///    * @param aOffset   The first character's offset.  0 is the first character.
    ///    * @param aLength   The length of getting text.  If the aLength is too long,
    ///    *                  the extra length is ignored.
    ///    * @param aX        Not used.
    ///    * @param aY        Not used.
    ///    */
    /// ```
    ///

    pub const QUERY_TEXT_RECT_ARRAY: i64 = 3209;

    /// ```text
    /// /**
    ///    * If sendQueryContentEvent()'s aAdditionalFlags argument is
    ///    * SELECTION_SET_FLAG_USE_NATIVE_LINE_BREAK, aOffset and aLength are offset
    ///    * and length in/of plain text generated from content is created with "\n".
    ///    * Otherwise, platform dependent.  E.g., on Windows, "\r\n" is used.
    ///    */
    /// ```
    ///

    pub const SELECTION_SET_FLAG_USE_NATIVE_LINE_BREAK: i64 = 0;


    pub const SELECTION_SET_FLAG_USE_XP_LINE_BREAK: i64 = 1;

    /// ```text
    /// /**
    ///    * If SELECTION_SET_FLAG_REVERSE is set, the selection is set from
    ///    * |aOffset + aLength| to |aOffset|.  Otherwise, it's set from |aOffset| to
    ///    * |aOffset + aLength|.
    ///    */
    /// ```
    ///

    pub const SELECTION_SET_FLAG_REVERSE: i64 = 2;


    pub const SELECT_CHARACTER: i64 = 0;


    pub const SELECT_CLUSTER: i64 = 1;


    pub const SELECT_WORD: i64 = 2;


    pub const SELECT_LINE: i64 = 3;


    pub const SELECT_BEGINLINE: i64 = 4;


    pub const SELECT_ENDLINE: i64 = 5;


    pub const SELECT_PARAGRAPH: i64 = 6;


    pub const SELECT_WORDNOSPACE: i64 = 7;

    /// ```text
    /// /**
    ///    * Returns all the audio input/output devices.
    ///    */
    /// ```
    ///

    pub const AUDIO_INPUT: i64 = 0;


    pub const AUDIO_OUTPUT: i64 = 1;


    pub const AGENT_SHEET: i64 = 0;


    pub const USER_SHEET: i64 = 1;


    pub const AUTHOR_SHEET: i64 = 2;


    pub const DEFAULT_MOUSE_POINTER_ID: i64 = 0;


    pub const DEFAULT_PEN_POINTER_ID: i64 = 1;


    pub const DEFAULT_TOUCH_POINTER_ID: i64 = 2;


    pub const MOUSE_BUTTON_LEFT_BUTTON: i64 = 0;


    pub const MOUSE_BUTTON_MIDDLE_BUTTON: i64 = 1;


    pub const MOUSE_BUTTON_RIGHT_BUTTON: i64 = 2;


    pub const MOUSE_BUTTONS_NO_BUTTON: i64 = 0;


    pub const MOUSE_BUTTONS_LEFT_BUTTON: i64 = 1;


    pub const MOUSE_BUTTONS_RIGHT_BUTTON: i64 = 2;


    pub const MOUSE_BUTTONS_MIDDLE_BUTTON: i64 = 4;


    pub const MOUSE_BUTTONS_4TH_BUTTON: i64 = 8;


    pub const MOUSE_BUTTONS_5TH_BUTTON: i64 = 16;


    pub const MOUSE_BUTTONS_NOT_SPECIFIED: i64 = -1;


    pub const DIRECTION_LTR: i64 = 0;


    pub const DIRECTION_RTL: i64 = 1;


    pub const DIRECTION_NOT_SET: i64 = 2;

    /// ```text
    /// /**
    ///    * Image animation mode of the window. When this attribute's value
    ///    * is changed, the implementation should set all images in the window
    ///    * to the given value. That is, when set to kDontAnimMode, all images
    ///    * will stop animating. The attribute's value must be one of the
    ///    * animationMode values from imgIContainer.
    ///    * @note Images may individually override the window's setting after
    ///    *       the window's mode is set. Therefore images given different modes
    ///    *       since the last setting of the window's mode may behave
    ///    *       out of line with the window's overall mode.
    ///    * @note The attribute's value is the window's overall mode. It may
    ///    *       for example continue to report kDontAnimMode after all images
    ///    *       have subsequently been individually animated.
    ///    * @note Only images immediately in this window are affected;
    ///    *       this is not recursive to subwindows.
    ///    * @see imgIContainer
    ///    */
    /// ```
    ///

    /// `attribute unsigned short imageAnimationMode;`
    #[inline]
    pub unsafe fn GetImageAnimationMode(&self, aImageAnimationMode: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetImageAnimationMode)(self, aImageAnimationMode)
    }


    /// ```text
    /// /**
    ///    * Image animation mode of the window. When this attribute's value
    ///    * is changed, the implementation should set all images in the window
    ///    * to the given value. That is, when set to kDontAnimMode, all images
    ///    * will stop animating. The attribute's value must be one of the
    ///    * animationMode values from imgIContainer.
    ///    * @note Images may individually override the window's setting after
    ///    *       the window's mode is set. Therefore images given different modes
    ///    *       since the last setting of the window's mode may behave
    ///    *       out of line with the window's overall mode.
    ///    * @note The attribute's value is the window's overall mode. It may
    ///    *       for example continue to report kDontAnimMode after all images
    ///    *       have subsequently been individually animated.
    ///    * @note Only images immediately in this window are affected;
    ///    *       this is not recursive to subwindows.
    ///    * @see imgIContainer
    ///    */
    /// ```
    ///

    /// `attribute unsigned short imageAnimationMode;`
    #[inline]
    pub unsafe fn SetImageAnimationMode(&self, aImageAnimationMode: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetImageAnimationMode)(self, aImageAnimationMode)
    }


    /// ```text
    /// /**
    ///    * Whether the charset of the window's current document has been forced by
    ///    * the user.
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean docCharsetIsForced;`
    #[inline]
    pub unsafe fn GetDocCharsetIsForced(&self, aDocCharsetIsForced: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDocCharsetIsForced)(self, aDocCharsetIsForced)
    }


    /// ```text
    /// /**
    ///    * Return the conversion of a physical millimeter in CSS pixels.
    ///    */
    /// ```
    ///

    /// `readonly attribute float physicalMillimeterInCSSPixels;`
    #[inline]
    pub unsafe fn GetPhysicalMillimeterInCSSPixels(&self, aPhysicalMillimeterInCSSPixels: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetPhysicalMillimeterInCSSPixels)(self, aPhysicalMillimeterInCSSPixels)
    }


    /// ```text
    /// /**
    ///    * Function to get metadata associated with the window's current document
    ///    * @param aName the name of the metadata.  This should be all lowercase.
    ///    * @return the value of the metadata, or the empty string if it's not set
    ///    *
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    */
    /// ```
    ///

    /// `AString getDocumentMetadata (in AString aName);`
    #[inline]
    pub unsafe fn GetDocumentMetadata(&self, aName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentMetadata)(self, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Force a synchronous layer transaction for this window if necessary.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void updateLayerTree ();`
    #[inline]
    pub unsafe fn UpdateLayerTree(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateLayerTree)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the last used layer transaction id for this window's refresh driver.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long lastTransactionId;`
    #[inline]
    pub unsafe fn GetLastTransactionId(&self, aLastTransactionId: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastTransactionId)(self, aLastTransactionId)
    }


    /// ```text
    /// /**
    ///    * Information retrieved from the <meta name="viewport"> tag.
    ///    * See Document::GetViewportInfo for more information.
    ///    */
    /// ```
    ///

    /// `void getViewportInfo (in uint32_t aDisplayWidth, in uint32_t aDisplayHeight, out double aDefaultZoom, out boolean aAllowZoom, out double aMinZoom, out double aMaxZoom, out uint32_t aWidth, out uint32_t aHeight, out boolean aAutoSize);`
    #[inline]
    pub unsafe fn GetViewportInfo(&self, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t, aDefaultZoom: *mut libc::c_double, aAllowZoom: *mut bool, aMinZoom: *mut libc::c_double, aMaxZoom: *mut libc::c_double, aWidth: *mut uint32_t, aHeight: *mut uint32_t, aAutoSize: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetViewportInfo)(self, aDisplayWidth, aDisplayHeight, aDefaultZoom, aAllowZoom, aMinZoom, aMaxZoom, aWidth, aHeight, aAutoSize)
    }



    /// `AString getViewportFitInfo ();`
    #[inline]
    pub unsafe fn GetViewportFitInfo(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetViewportFitInfo)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Information about the window size in device pixels.
    ///    */
    /// ```
    ///

    /// `void getContentViewerSize (out uint32_t aDisplayWidth, out uint32_t aDisplayHeight);`
    #[inline]
    pub unsafe fn GetContentViewerSize(&self, aDisplayWidth: *mut uint32_t, aDisplayHeight: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetContentViewerSize)(self, aDisplayWidth, aDisplayHeight)
    }


    /// ```text
    /// /**
    ///    * For any scrollable element, this allows you to override the
    ///    * visible region and draw more than what is visible, which is
    ///    * useful for asynchronous drawing. The "displayport" will be
    ///    * <xPx, yPx, widthPx, heightPx> in units of CSS pixels,
    ///    * regardless of the size of the enclosing container.  This
    ///    * will *not* trigger reflow.
    ///    *
    ///    * For the root scroll area, pass in the root document element.
    ///    * For scrollable elements, pass in the container element (for
        ///    * instance, the element with overflow: scroll).
    ///    *
    ///    * <x, y> is relative to the top-left of what would normally be
    ///    * the visible area of the element. This means that the pixels
    ///    * rendered to the displayport take scrolling into account,
    ///    * for example.
    ///    *
    ///    * It's legal to set a displayport that extends beyond the overflow
    ///    * area in any direction (left/right/top/bottom).
    ///    *
    ///    * It's also legal to set a displayport that extends beyond the
    ///    * area's bounds.  No pixels are rendered outside the area bounds.
    ///    *
    ///    * The caller of this method must have chrome privileges.
    ///    *
    ///    * Calling this will always force a recomposite, so it should be
    ///    * avoided if at all possible. Client code should do checks before
    ///    * calling this so that duplicate sets are not made with the same
    ///    * displayport.
    ///    *
    ///    * aPriority is recorded along with the displayport rectangle. If this
    ///    * method is called with a lower priority than the current priority, the
    ///    * call is ignored.
    ///    */
    /// ```
    ///

    /// `void setDisplayPortForElement (in float aXPx, in float aYPx, in float aWidthPx, in float aHeightPx, in Element aElement, in uint32_t aPriority);`
    #[inline]
    pub unsafe fn SetDisplayPortForElement(&self, aXPx: libc::c_float, aYPx: libc::c_float, aWidthPx: libc::c_float, aHeightPx: libc::c_float, aElement: *const libc::c_void, aPriority: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplayPortForElement)(self, aXPx, aYPx, aWidthPx, aHeightPx, aElement, aPriority)
    }


    /// ```text
    /// /**
    ///    * An alternate way to represent a displayport rect as a set of margins and a
    ///    * base rect to apply those margins to. A consumer of pixels may ask for as
    ///    * many extra pixels as it would like in each direction. Layout then sets
    ///    * the base rect to the "visible rect" of the element, which is just the
    ///    * subrect of the element that is drawn (it does not take in account content
        ///    * covering the element).
    ///    *
    ///    * If both a displayport rect and displayport margins with corresponding base
    ///    * rect are set with the same priority then the margins will take precendence.
    ///    *
    ///    * Specifying an alignment value will ensure that after the base rect has
    ///    * been expanded by the displayport margins, it will be further expanded so
    ///    * that each edge is located at a multiple of the "alignment" value.
    ///    *
    ///    * Note that both the margin values and alignment are treated as values in
    ///    * ScreenPixels. Refer to layout/base/Units.h for a description of this unit.
    ///    * The base rect values are in app units.
    ///    */
    /// ```
    ///

    /// `void setDisplayPortMarginsForElement (in float aLeftMargin, in float aTopMargin, in float aRightMargin, in float aBottomMargin, in Element aElement, in uint32_t aPriority);`
    #[inline]
    pub unsafe fn SetDisplayPortMarginsForElement(&self, aLeftMargin: libc::c_float, aTopMargin: libc::c_float, aRightMargin: libc::c_float, aBottomMargin: libc::c_float, aElement: *const libc::c_void, aPriority: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplayPortMarginsForElement)(self, aLeftMargin, aTopMargin, aRightMargin, aBottomMargin, aElement, aPriority)
    }



    /// `void setDisplayPortBaseForElement (in int32_t aX, in int32_t aY, in int32_t aWidth, in int32_t aHeight, in Element aElement);`
    #[inline]
    pub unsafe fn SetDisplayPortBaseForElement(&self, aX: int32_t, aY: int32_t, aWidth: int32_t, aHeight: int32_t, aElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplayPortBaseForElement)(self, aX, aY, aWidth, aHeight, aElement)
    }


    /// ```text
    /// /**
    ///    * If |aElement| is a scroll container, returns the amount of layout
    ///    * space taken up by its scrollbars (that is, the width of the vertical
        ///    * scrollbar and the height of the horizontal scrollbar) in CSS pixels;
    ///    * otherwise returns zero.
    ///    *
    ///    * Note that on some platforms, scrollbars don't take up layout space
    ///    * ("overlay scrollbars"). On such platforms, the returned sizes are
    ///    * always zero.
    ///    *
    ///    * Layout scrollbars that normally take up space but were only shown to
    ///    * scroll the visual viewport inside the layout viewport (the layout viewport
        ///    * cannot be scrolled) do not take up space but they still return their size
    ///    * from this function.
    ///    */
    /// ```
    ///

    /// `void getScrollbarSizes (in Element aElement, out uint32_t aVerticalScrollbarWidth, out uint32_t aHorizontalScrollbarHeight);`
    #[inline]
    pub unsafe fn GetScrollbarSizes(&self, aElement: *const libc::c_void, aVerticalScrollbarWidth: *mut uint32_t, aHorizontalScrollbarHeight: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollbarSizes)(self, aElement, aVerticalScrollbarWidth, aHorizontalScrollbarHeight)
    }


    /// ```text
    /// /**
    ///    * Get/set the resolution at which rescalable web content is drawn for
    ///    * testing purposes.
    ///    *
    ///    * Setting a new resolution does *not* trigger reflow.  This API is
    ///    * entirely separate from textZoom and fullZoom; a resolution scale
    ///    * can be applied together with both textZoom and fullZoom.
    ///    *
    ///    * The effect of this API is for gfx code to allocate more or fewer
    ///    * pixels for rescalable content by a factor of |resolution| in
    ///    * both dimensions.
    ///    *
    ///    * In addition, the content is scaled by the amount of the resolution,
    ///    * so that it is displayed at a correspondingly larger or smaller size,
    ///    * without the need for the caller to set an additional transform.
    ///    *
    ///    * The purpose of this API is to allow tests to simulate many of the effects
    ///    * a non-reflowing scale-zoom, e.g. for pinch-zoom on mobile platforms, and
    ///    * should be only used for testing purposes.
    ///    *
    ///    * The caller of this method must have chrome privileges.
    ///    *
    ///    * This is intended to be used by test code only!
    ///    */
    /// ```
    ///

    /// `void setResolutionAndScaleTo (in float aResolution);`
    #[inline]
    pub unsafe fn SetResolutionAndScaleTo(&self, aResolution: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetResolutionAndScaleTo)(self, aResolution)
    }



    /// `float getResolution ();`
    #[inline]
    pub unsafe fn GetResolution(&self, _retval: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetResolution)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Set a resolution on the presShell which is the "restored" from history.
    ///    * The display dimensions are compared to their current values and used
    ///    * to scale the resolution value if necessary, e.g. if the device was
    ///    * rotated between saving and restoring of the session data.
    ///    * This resolution should be used when painting for the first time. Calling
    ///    * this too late may have no effect.
    ///    */
    /// ```
    ///

    /// `void setRestoreResolution (in float aResolution, in uint32_t aDisplayWidth, in uint32_t aDisplayHeight);`
    #[inline]
    pub unsafe fn SetRestoreResolution(&self, aResolution: libc::c_float, aDisplayWidth: uint32_t, aDisplayHeight: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetRestoreResolution)(self, aResolution, aDisplayWidth, aDisplayHeight)
    }


    /// ```text
    /// /**
    ///    * Whether the next paint should be flagged as the first paint for a document.
    ///    * This gives a way to track the next paint that occurs after the flag is
    ///    * set. The flag gets cleared after the next paint.
    ///    *
    ///    * Can only be accessed with chrome privileges.
    ///    */
    /// ```
    ///

    /// `attribute boolean isFirstPaint;`
    #[inline]
    pub unsafe fn GetIsFirstPaint(&self, aIsFirstPaint: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFirstPaint)(self, aIsFirstPaint)
    }


    /// ```text
    /// /**
    ///    * Whether the next paint should be flagged as the first paint for a document.
    ///    * This gives a way to track the next paint that occurs after the flag is
    ///    * set. The flag gets cleared after the next paint.
    ///    *
    ///    * Can only be accessed with chrome privileges.
    ///    */
    /// ```
    ///

    /// `attribute boolean isFirstPaint;`
    #[inline]
    pub unsafe fn SetIsFirstPaint(&self, aIsFirstPaint: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsFirstPaint)(self, aIsFirstPaint)
    }



    /// `uint32_t getPresShellId ();`
    #[inline]
    pub unsafe fn GetPresShellId(&self, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPresShellId)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns whether a given header and value is a CORS-safelisted request
    ///    * header per https://fetch.spec.whatwg.org/#cors-safelisted-request-header
    ///    */
    /// ```
    ///

    /// `boolean isCORSSafelistedRequestHeader (in ACString name, in ACString value);`
    #[inline]
    pub unsafe fn IsCORSSafelistedRequestHeader(&self, name: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCORSSafelistedRequestHeader)(self, name, value, _retval)
    }


    /// ```text
    /// /** Synthesize a mouse event. The event types supported are:
    ///    *    mousedown, mouseup, mousemove, mouseover, mouseout, mousecancel,
    ///    *    contextmenu, MozMouseHittest
    ///    *
    ///    * Events are sent in coordinates offset by aX and aY from the window.
    ///    *
    ///    * Note that additional events may be fired as a result of this call. For
    ///    * instance, typically a click event will be fired as a result of a
    ///    * mousedown and mouseup in sequence.
    ///    *
    ///    * Normally at this level of events, the mouseover and mouseout events are
    ///    * only fired when the window is entered or exited. For inter-element
    ///    * mouseover and mouseout events, a movemove event fired on the new element
    ///    * should be sufficient to generate the correct over and out events as well.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * The event is dispatched via the toplevel window, so it could go to any
    ///    * window under the toplevel window, in some cases it could never reach this
    ///    * window at all.
    ///    *
    ///    * NOTE: mousecancel is used to represent the vanishing of an input device
    ///    * such as a pen leaving its digitizer by synthesizing a WidgetMouseEvent,
    ///    * whose mMessage is eMouseExitFromWidget and mExitFrom is
    ///    * WidgetMouseEvent::eTopLevel.
    ///    *
    ///    * @param aType event type
    ///    * @param aX x offset in CSS pixels
    ///    * @param aY y offset in CSS pixels
    ///    * @param aButton button to synthesize
    ///    * @param aClickCount number of clicks that have been performed
    ///    * @param aModifiers modifiers pressed, using constants defined as MODIFIER_*
    ///    * @param aIgnoreRootScrollFrame whether the event should ignore viewport bounds
    ///    *                           during dispatch
    ///    * @param aPressure touch input pressure: 0.0 -> 1.0
    ///    * @param aInputSourceArg input source, see MouseEvent for values,
    ///    *        defaults to mouse input.
    ///    * @param aIsDOMEventSynthesized controls Event.isSynthesized value
    ///    *                               that helps identifying test related events,
    ///    *                               defaults to true
    ///    * @param aIsWidgetEventSynthesized controls WidgetMouseEvent.mReason value
    ///    *                                  defaults to false (WidgetMouseEvent::eReal)
    ///    * @param aIdentifier A unique identifier for the pointer causing the event,
    ///    *                    defaulting to nsIDOMWindowUtils::DEFAULT_MOUSE_POINTER_ID.
    ///    *
    ///    * returns true if the page called prevent default on this event
    ///    */
    /// ```
    ///

    /// `[can_run_script,optional_argc] boolean sendMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier);`
    const _SendMouseEvent: () = ();

    /// ```text
    /// /** Synthesize a touch event. The event types supported are:
    ///    *    touchstart, touchend, touchmove, and touchcancel
    ///    *
    ///    * Events are sent in coordinates offset by aX and aY from the window.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * The event is dispatched via the toplevel window, so it could go to any
    ///    * window under the toplevel window, in some cases it could never reach this
    ///    * window at all.
    ///    *
    ///    * @param aType event type
    ///    * @param xs array of offsets in CSS pixels for each touch to be sent
    ///    * @param ys array of offsets in CSS pixels for each touch to be sent
    ///    * @param rxs array of radii in CSS pixels for each touch to be sent
    ///    * @param rys array of radii in CSS pixels for each touch to be sent
    ///    * @param rotationAngles array of angles in degrees for each touch to be sent
    ///    * @param forces array of forces (floats from 0 to 1) for each touch to be sent
    ///    * @param count number of touches in this set
    ///    * @param aModifiers modifiers pressed, using constants defined as MODIFIER_*
    ///    * @param aIgnoreRootScrollFrame whether the event should ignore viewport bounds
    ///    *                           during dispatch
    ///    *
    ///    * returns true if the page called prevent default on this touch event
    ///    */
    /// ```
    ///

    /// `[can_run_script] boolean sendTouchEvent (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame);`
    #[inline]
    pub unsafe fn SendTouchEvent(&self, aType: *const ::nsstring::nsAString, aIdentifiers: *const thin_vec::ThinVec<uint32_t>, aXs: *const thin_vec::ThinVec<int32_t>, aYs: *const thin_vec::ThinVec<int32_t>, aRxs: *const thin_vec::ThinVec<uint32_t>, aRys: *const thin_vec::ThinVec<uint32_t>, aRotationAngles: *const thin_vec::ThinVec<libc::c_float>, aForces: *const thin_vec::ThinVec<libc::c_float>, aModifiers: i32, aIgnoreRootScrollFrame: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SendTouchEvent)(self, aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval)
    }


    /// ```text
    /// /** The same as sendMouseEvent but ensures that the event is dispatched to
    ///    *  this DOM window or one of its children.
    ///    */
    /// ```
    ///

    /// `[can_run_script,optional_argc] void sendMouseEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier);`
    const _SendMouseEventToWindow: () = ();

    /// ```text
    /// /** The same as sendTouchEvent but ensures that the event is dispatched to
    ///    *  this DOM window or one of its children.
    ///    */
    /// ```
    ///

    /// `[can_run_script] boolean sendTouchEventToWindow (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame);`
    #[inline]
    pub unsafe fn SendTouchEventToWindow(&self, aType: *const ::nsstring::nsAString, aIdentifiers: *const thin_vec::ThinVec<uint32_t>, aXs: *const thin_vec::ThinVec<int32_t>, aYs: *const thin_vec::ThinVec<int32_t>, aRxs: *const thin_vec::ThinVec<uint32_t>, aRys: *const thin_vec::ThinVec<uint32_t>, aRotationAngles: *const thin_vec::ThinVec<libc::c_float>, aForces: *const thin_vec::ThinVec<libc::c_float>, aModifiers: i32, aIgnoreRootScrollFrame: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SendTouchEventToWindow)(self, aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval)
    }



    /// `void sendWheelEvent (in float aX, in float aY, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aDeltaMode, in long aModifiers, in long aLineOrPageDeltaX, in long aLineOrPageDeltaY, in unsigned long aOptions);`
    #[inline]
    pub unsafe fn SendWheelEvent(&self, aX: libc::c_float, aY: libc::c_float, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aDeltaMode: u32, aModifiers: i32, aLineOrPageDeltaX: i32, aLineOrPageDeltaY: i32, aOptions: u32) -> ::nserror::nsresult {
        ((*self.vtable).SendWheelEvent)(self, aX, aY, aDeltaX, aDeltaY, aDeltaZ, aDeltaMode, aModifiers, aLineOrPageDeltaX, aLineOrPageDeltaY, aOptions)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::SynthesizeNativeKeyEvent
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * When you use this for tests, use the constants defined in NativeKeyCodes.js
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified with a "keyevent"
    ///    * topic.
    ///    */
    /// ```
    ///

    /// `void sendNativeKeyEvent (in long aNativeKeyboardLayout, in long aNativeKeyCode, in long aModifierFlags, in AString aCharacters, in AString aUnmodifiedCharacters, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeKeyEvent(&self, aNativeKeyboardLayout: i32, aNativeKeyCode: i32, aModifierFlags: i32, aCharacters: *const ::nsstring::nsAString, aUnmodifiedCharacters: *const ::nsstring::nsAString, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeKeyEvent)(self, aNativeKeyboardLayout, aNativeKeyCode, aModifierFlags, aCharacters, aUnmodifiedCharacters, aObserver)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::SynthesizeNativeMouseEvent
    ///    *
    ///    * Will be called on the widget that contains aElement.
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified with a "mouseevent"
    ///    * topic.
    ///    */
    /// ```
    ///

    /// `void sendNativeMouseEvent (in long aScreenX, in long aScreenY, in long aNativeMessage, in long aModifierFlags, in Element aElement, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeMouseEvent(&self, aScreenX: i32, aScreenY: i32, aNativeMessage: i32, aModifierFlags: i32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeMouseEvent)(self, aScreenX, aScreenY, aNativeMessage, aModifierFlags, aElement, aObserver)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::SynthesizeNativeMouseMove and sendNativeMouseEvent
    ///    */
    /// ```
    ///

    /// `void sendNativeMouseMove (in long aScreenX, in long aScreenY, in Element aElement, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeMouseMove(&self, aScreenX: i32, aScreenY: i32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeMouseMove)(self, aScreenX, aScreenY, aElement, aObserver)
    }


    /// ```text
    /// /**
    ///    * Suppress animations that are applied to a window by OS when
    ///    * resizing, moving, changing size mode, ...
    ///    */
    /// ```
    ///

    /// `void suppressAnimation (in boolean aSuppress);`
    #[inline]
    pub unsafe fn SuppressAnimation(&self, aSuppress: bool) -> ::nserror::nsresult {
        ((*self.vtable).SuppressAnimation)(self, aSuppress)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::SynthesizeNativeMouseScrollEvent
    ///    *
    ///    * Will be called on the widget that contains aElement.
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified with a
    ///    * "mousescrollevent" topic.
    ///    *
    ///    * @param aNativeMessage
    ///    *   On Windows:  WM_MOUSEWHEEL (0x020A), WM_MOUSEHWHEEL(0x020E),
    ///    *                WM_VSCROLL (0x0115) or WM_HSCROLL (0x114).
    ///    */
    /// ```
    ///

    /// `void sendNativeMouseScrollEvent (in long aScreenX, in long aScreenY, in unsigned long aNativeMessage, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aModifierFlags, in unsigned long aAdditionalFlags, in Element aElement, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeMouseScrollEvent(&self, aScreenX: i32, aScreenY: i32, aNativeMessage: u32, aDeltaX: libc::c_double, aDeltaY: libc::c_double, aDeltaZ: libc::c_double, aModifierFlags: u32, aAdditionalFlags: u32, aElement: *const libc::c_void, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeMouseScrollEvent)(self, aScreenX, aScreenY, aNativeMessage, aDeltaX, aDeltaY, aDeltaZ, aModifierFlags, aAdditionalFlags, aElement, aObserver)
    }


    /// ```text
    /// /**
    ///    * Create a new or update an existing touch point on the digitizer.
    ///    * To trigger os level gestures, individual touch points should
    ///    * transition through a complete set of touch states which should be
    ///    * sent as individual calls. For example:
    ///    * tap - msg1:TOUCH_CONTACT, msg2:TOUCH_REMOVE
    ///    * drag - msg1-n:TOUCH_CONTACT (moving), msgn+1:TOUCH_REMOVE
    ///    * hover drag - msg1-n:TOUCH_HOVER (moving), msgn+1:TOUCH_REMOVE
    ///    *
    ///    * Widget support: Windows 8.0+, Winrt/Win32. Other widgets will throw.
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified with a "touchpoint"
    ///    * topic.
    ///    *
    ///    * @param aPointerId The touch point id to create or update.
    ///    * @param aTouchState one or more of the touch states listed above
    ///    * @param aScreenX, aScreenY screen coords of this event
    ///    * @param aPressure 0.0 -> 1.0 float val indicating pressure
    ///    * @param aOrientation 0 -> 359 degree value indicating the
    ///    * orientation of the pointer. Use 90 for normal taps.
    ///    */
    /// ```
    ///

    /// `void sendNativeTouchPoint (in unsigned long aPointerId, in unsigned long aTouchState, in long aScreenX, in long aScreenY, in double aPressure, in unsigned long aOrientation, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeTouchPoint(&self, aPointerId: u32, aTouchState: u32, aScreenX: i32, aScreenY: i32, aPressure: libc::c_double, aOrientation: u32, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeTouchPoint)(self, aPointerId, aTouchState, aScreenX, aScreenY, aPressure, aOrientation, aObserver)
    }


    /// ```text
    /// /**
    ///    * These values indicate touchpad pinch phase states :
    ///    * PHASE_BEGIN
    ///    * PHASE_UPDATE
    ///    * PHASE_END
    ///    * Widget support: Linux GTK 3.18+.
    ///    * @param aEventPhase The touchpad pinch phase using states listed above.
    ///    * @param aScale Events with PHASE_UPDATE will change the zoom level by
    ///    * the ratio between the scale of the current event and the scale of the last event.
    ///    * @param aScreenX, aScreenY screen coords of the focus point of this event.
    ///    * @param aModifierFlags is expected to contain native modifier values.
    ///    */
    /// ```
    ///

    /// `void sendNativeTouchpadPinch (in unsigned long aEventPhase, in float aScale, in long aScreenX, in long aScreenY, in long aModifierFlags);`
    #[inline]
    pub unsafe fn SendNativeTouchpadPinch(&self, aEventPhase: u32, aScale: libc::c_float, aScreenX: i32, aScreenY: i32, aModifierFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeTouchpadPinch)(self, aEventPhase, aScale, aScreenX, aScreenY, aModifierFlags)
    }


    /// ```text
    /// /**
    ///    * Simulates native touch based taps on the input digitizer. Events
    ///    * triggered by this call are injected at the os level. Events do not
    ///    * bypass widget level input processing and as such can be used to
    ///    * test widget event logic and async pan-zoom controller functionality.
    ///    * Cannot be accessed from an unprivileged context.
    ///    *
    ///    * Long taps (based on the aLongTap parameter) will be completed
    ///    * asynchrnously after the call returns. Long tap delay is based on
    ///    * the ui.click_hold_context_menus.delay pref or 1500 msec if pref
    ///    * is not set.
    ///    *
    ///    * Widget support: Windows 8.0+, Winrt/Win32. Other widgets will
    ///    * throw.
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified, with a "touchtap"
    ///    * topic.
    ///    *
    ///    * @param aScreenX, aScreenY screen coords of this event
    ///    * @param aLongTap true if the tap should be long, false for a short
    ///    * tap.
    ///    */
    /// ```
    ///

    /// `void sendNativeTouchTap (in long aScreenX, in long aScreenY, in boolean aLongTap, [optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn SendNativeTouchTap(&self, aScreenX: i32, aScreenY: i32, aLongTap: bool, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).SendNativeTouchTap)(self, aScreenX, aScreenY, aLongTap, aObserver)
    }


    /// ```text
    /// /**
    ///    * Cancel any existing touch points or long tap delays. Calling this is safe
    ///    * even if you're sure there aren't any pointers recorded. You should call
    ///    * this when tests shut down to reset the digitizer driver. Not doing so can
    ///    * leave the digitizer in an undetermined state which can screw up subsequent
    ///    * tests and native input.
    ///    *
    ///    * NOTE: The synthesized native event will be fired asynchronously, and upon
    ///    * completion the observer, if provided, will be notified with a "cleartouch"
    ///    * topic.
    ///    */
    /// ```
    ///

    /// `void clearNativeTouchSequence ([optional] in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn ClearNativeTouchSequence(&self, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).ClearNativeTouchSequence)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Clears the SharedStyleSheetCache.
    ///    */
    /// ```
    ///

    /// `void clearSharedStyleSheetCache ();`
    #[inline]
    pub unsafe fn ClearSharedStyleSheetCache(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearSharedStyleSheetCache)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns the number of stylesheets that have been parsed on this document.
    ///    * Useful to test caching.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long parsedStyleSheets;`
    #[inline]
    pub unsafe fn GetParsedStyleSheets(&self, aParsedStyleSheets: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetParsedStyleSheets)(self, aParsedStyleSheets)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::ActivateNativeMenuItemAt
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    */
    /// ```
    ///

    /// `void activateNativeMenuItemAt (in AString indexString);`
    #[inline]
    pub unsafe fn ActivateNativeMenuItemAt(&self, indexString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ActivateNativeMenuItemAt)(self, indexString)
    }


    /// ```text
    /// /**
    ///    * See nsIWidget::ForceUpdateNativeMenuAt
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    */
    /// ```
    ///

    /// `void forceUpdateNativeMenuAt (in AString indexString);`
    #[inline]
    pub unsafe fn ForceUpdateNativeMenuAt(&self, indexString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ForceUpdateNativeMenuAt)(self, indexString)
    }


    /// ```text
    /// /**
    ///    * Returns the current selection as plaintext. Note that the result may be
    ///    * different from the result of sendQueryContentEvent(QUERY_SELECTED_TEXT).
    ///    * This result is computed by native API with transferable data. In other
    ///    * words, when the OS treats the selection as plaintext, it treats current
    ///    * selection as this result.
    ///    */
    /// ```
    ///

    /// `AString GetSelectionAsPlaintext ();`
    #[inline]
    pub unsafe fn GetSelectionAsPlaintext(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionAsPlaintext)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Force a garbage collection followed by a cycle collection.
    ///    *
    ///    * Will throw a DOM security error if called without chrome privileges in
    ///    * non-debug builds. Available to all callers in debug builds.
    ///    *
    ///    * @param aListener listener that receives information about the CC graph
    ///    */
    /// ```
    ///

    /// `void garbageCollect ([optional] in nsICycleCollectorListener aListener);`
    #[inline]
    pub unsafe fn GarbageCollect(&self, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult {
        ((*self.vtable).GarbageCollect)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Force a cycle collection without garbage collection.
    ///    *
    ///    * Will throw a DOM security error if called without chrome privileges in
    ///    * non-debug builds. Available to all callers in debug builds.
    ///    *
    ///    * @param aListener listener that receives information about the CC graph
    ///    */
    /// ```
    ///

    /// `void cycleCollect ([optional] in nsICycleCollectorListener aListener);`
    #[inline]
    pub unsafe fn CycleCollect(&self, aListener: *const nsICycleCollectorListener) -> ::nserror::nsresult {
        ((*self.vtable).CycleCollect)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Trigger whichever GC or CC timer is currently active and waiting to fire.
    ///    * Don't do this too much for initiating heavy actions, like the start of a IGC.
    ///    */
    /// ```
    ///

    /// `void runNextCollectorTimer ();`
    #[inline]
    pub unsafe fn RunNextCollectorTimer(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RunNextCollectorTimer)(self, )
    }


    /// ```text
    /// /** Synthesize a simple gesture event for a window. The event types
    ///    *  supported are: MozSwipeGestureMayStart, MozSwipeGestureStart,
    ///    *  MozSwipeGestureUpdate, MozSwipeGestureEnd, MozSwipeGesture,
    ///    *  MozMagnifyGestureStart, MozMagnifyGestureUpdate, MozMagnifyGesture,
    ///    *  MozRotateGestureStart, MozRotateGestureUpdate, MozRotateGesture,
    ///    *  MozPressTapGesture, MozTapGesture, and MozEdgeUIGesture.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not
        ///    * content-accessible) Will throw a DOM security error if called
    ///    * without chrome privileges.
    ///    *
    ///    * @param aType event type
    ///    * @param aX x offset in CSS pixels
    ///    * @param aY y offset in CSS pixels
    ///    * @param aDirection direction, using constants defined in SimpleGestureEvent.webidl
    ///    * @param aDelta  amount of magnification or rotation for magnify and rotation events
    ///    * @param aModifiers modifiers pressed, using constants defined in Event.webidl
    ///    * @param aClickCount For tap gestures, the number of taps.
    ///    */
    /// ```
    ///

    /// `void sendSimpleGestureEvent (in AString aType, in float aX, in float aY, in unsigned long aDirection, in double aDelta, in long aModifiers, [optional] in unsigned long aClickCount);`
    #[inline]
    pub unsafe fn SendSimpleGestureEvent(&self, aType: *const ::nsstring::nsAString, aX: libc::c_float, aY: libc::c_float, aDirection: u32, aDelta: libc::c_double, aModifiers: i32, aClickCount: u32) -> ::nserror::nsresult {
        ((*self.vtable).SendSimpleGestureEvent)(self, aType, aX, aY, aDirection, aDelta, aModifiers, aClickCount)
    }


    /// ```text
    /// /**
    ///    * Retrieve the element at point aX, aY in the window's document.
    ///    *
    ///    * @param aIgnoreRootScrollFrame whether or not to ignore the root scroll
    ///    *        frame when retrieving the element. If false, this method returns
    ///    *        null for coordinates outside of the viewport.
    ///    * @param aFlushLayout flushes layout if true. Otherwise, no flush occurs.
    ///    */
    /// ```
    ///

    /// `Element elementFromPoint (in float aX, in float aY, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout);`
    #[inline]
    pub unsafe fn ElementFromPoint(&self, aX: libc::c_float, aY: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ElementFromPoint)(self, aX, aY, aIgnoreRootScrollFrame, aFlushLayout, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve all nodes that intersect a rect in the window's document.
    ///    *
    ///    * @param aX x reference for the rectangle in CSS pixels
    ///    * @param aY y reference for the rectangle in CSS pixels
    ///    * @param aTopSize How much to expand up the rectangle
    ///    * @param aRightSize How much to expand right the rectangle
    ///    * @param aBottomSize How much to expand down the rectangle
    ///    * @param aLeftSize How much to expand left the rectangle
    ///    * @param aIgnoreRootScrollFrame whether or not to ignore the root scroll
    ///    *        frame when retrieving the element. If false, this method returns
    ///    *        null for coordinates outside of the viewport.
    ///    * @param aFlushLayout flushes layout if true. Otherwise, no flush occurs.
    ///    * @param aOnlyVisible Set to true if you only want nodes that pass a visibility
    ///    *        hit test.
    ///    * @param aTransparencyThreshold Only has an effect if aOnlyVisible is true.
    ///    *        Returns what amount of transparency is considered "opaque enough"
    ///    *        to consider elements "not visible". The default is effectively "1"
    ///    *        (so, only opaque elements will stop an element from being
        ///    *        "visible").
    ///    */
    /// ```
    ///

    /// `NodeList nodesFromRect (in float aX, in float aY, in float aTopSize, in float aRightSize, in float aBottomSize, in float aLeftSize, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout, in boolean aOnlyVisible, [optional] in float aTransparencyThreshold);`
    #[inline]
    pub unsafe fn NodesFromRect(&self, aX: libc::c_float, aY: libc::c_float, aTopSize: libc::c_float, aRightSize: libc::c_float, aBottomSize: libc::c_float, aLeftSize: libc::c_float, aIgnoreRootScrollFrame: bool, aFlushLayout: bool, aOnlyVisible: bool, aTransparencyThreshold: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).NodesFromRect)(self, aX, aY, aTopSize, aRightSize, aBottomSize, aLeftSize, aIgnoreRootScrollFrame, aFlushLayout, aOnlyVisible, aTransparencyThreshold, _retval)
    }


    /// ```text
    /// /**
    ///    * Get a list of nodes that have meaningful textual content to
    ///    * be translated. The implementation of this algorithm is in flux
    ///    * as we experiment and refine which approach works best.
    ///    *
    ///    * This method requires chrome privileges.
    ///    */
    /// ```
    ///

    /// `nsITranslationNodeList getTranslationNodes (in Node aRoot);`
    #[inline]
    pub unsafe fn GetTranslationNodes(&self, aRoot: *const libc::c_void, _retval: *mut*const nsITranslationNodeList) -> ::nserror::nsresult {
        ((*self.vtable).GetTranslationNodes)(self, aRoot, _retval)
    }


    /// ```text
    /// /**
    ///    * Compare the two canvases, returning the number of differing pixels and
    ///    * the maximum difference in a channel.  This will throw an error if
    ///    * the dimensions of the two canvases are different.
    ///    *
    ///    * This method requires chrome privileges.
    ///    */
    /// ```
    ///

    /// `uint32_t compareCanvases (in nsISupports aCanvas1, in nsISupports aCanvas2, out unsigned long aMaxDifference);`
    #[inline]
    pub unsafe fn CompareCanvases(&self, aCanvas1: *const nsISupports, aCanvas2: *const nsISupports, aMaxDifference: *mut u32, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).CompareCanvases)(self, aCanvas1, aCanvas2, aMaxDifference, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if a MozAfterPaint event has been queued but not yet
    ///    * fired.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isMozAfterPaintPending;`
    #[inline]
    pub unsafe fn GetIsMozAfterPaintPending(&self, aIsMozAfterPaintPending: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsMozAfterPaintPending)(self, aIsMozAfterPaintPending)
    }


    /// ```text
    /// /**
    ///    * Returns true if the InputTaskManager is suspended.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isInputTaskManagerSuspended;`
    #[inline]
    pub unsafe fn GetIsInputTaskManagerSuspended(&self, aIsInputTaskManagerSuspended: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInputTaskManagerSuspended)(self, aIsInputTaskManagerSuspended)
    }


    /// ```text
    /// /**
    ///    * Suppresses/unsuppresses user initiated event handling in window's document
    ///    * and subdocuments.
    ///    *
    ///    * @throw NS_ERROR_DOM_SECURITY_ERR if called without chrome privileges and
    ///    *        NS_ERROR_FAILURE if window doesn't have a document.
    ///    */
    /// ```
    ///

    /// `void suppressEventHandling (in boolean aSuppress);`
    #[inline]
    pub unsafe fn SuppressEventHandling(&self, aSuppress: bool) -> ::nserror::nsresult {
        ((*self.vtable).SuppressEventHandling)(self, aSuppress)
    }


    /// ```text
    /// /**
    ///    * Disable or enable non synthetic test mouse events on *all* windows.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible).
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * @param aDisable  If true, disable all non synthetic test mouse events
    ///    *               on all windows.  Otherwise, enable them.
    ///    */
    /// ```
    ///

    /// `void disableNonTestMouseEvents (in boolean aDisable);`
    #[inline]
    pub unsafe fn DisableNonTestMouseEvents(&self, aDisable: bool) -> ::nserror::nsresult {
        ((*self.vtable).DisableNonTestMouseEvents)(self, aDisable)
    }


    /// ```text
    /// /**
    ///    * Returns the scroll position of the window's currently loaded document.
    ///    *
    ///    * @param aFlushLayout flushes layout if true. Otherwise, no flush occurs.
    ///    * @see nsIDOMWindow::scrollX/Y
    ///    */
    /// ```
    ///

    /// `void getScrollXY (in boolean aFlushLayout, out long aScrollX, out long aScrollY);`
    #[inline]
    pub unsafe fn GetScrollXY(&self, aFlushLayout: bool, aScrollX: *mut i32, aScrollY: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollXY)(self, aFlushLayout, aScrollX, aScrollY)
    }


    /// ```text
    /// /**
    ///    * Returns the scroll position of the window's currently loaded document.
    ///    *
    ///    * @param aFlushLayout flushes layout if true. Otherwise, no flush occurs.
    ///    * @see nsIDOMWindow::scrollX/Y
    ///    */
    /// ```
    ///

    /// `void getScrollXYFloat (in boolean aFlushLayout, out float aScrollX, out float aScrollY);`
    #[inline]
    pub unsafe fn GetScrollXYFloat(&self, aFlushLayout: bool, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollXYFloat)(self, aFlushLayout, aScrollX, aScrollY)
    }


    /// ```text
    /// /**
    ///    * Returns the scrollbar width of the window's scroll frame.
    ///    *
    ///    * @param aFlushLayout flushes layout if true. Otherwise, no flush occurs.
    ///    */
    /// ```
    ///

    /// `void getScrollbarSize (in boolean aFlushLayout, out long aWidth, out long aHeight);`
    #[inline]
    pub unsafe fn GetScrollbarSize(&self, aFlushLayout: bool, aWidth: *mut i32, aHeight: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollbarSize)(self, aFlushLayout, aWidth, aHeight)
    }


    /// ```text
    /// /**
    ///    * Returns the given element's bounds without flushing pending layout changes.
    ///    */
    /// ```
    ///

    /// `DOMRect getBoundsWithoutFlushing (in Element aElement);`
    #[inline]
    pub unsafe fn GetBoundsWithoutFlushing(&self, aElement: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetBoundsWithoutFlushing)(self, aElement, _retval)
    }



    /// `void scrollToVisual (in float aOffsetX, in float aOffsetY, in long aUpdateType, in long aScrollMode);`
    #[inline]
    pub unsafe fn ScrollToVisual(&self, aOffsetX: libc::c_float, aOffsetY: libc::c_float, aUpdateType: i32, aScrollMode: i32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollToVisual)(self, aOffsetX, aOffsetY, aUpdateType, aScrollMode)
    }


    /// ```text
    /// /**
    ///    * Returns the offset of the window's visual viewport relative to the
    ///    * layout viewport.
    ///    */
    /// ```
    ///

    /// `void getVisualViewportOffsetRelativeToLayoutViewport (out float aOffsetX, out float aOffsetY);`
    #[inline]
    pub unsafe fn GetVisualViewportOffsetRelativeToLayoutViewport(&self, aOffsetX: *mut libc::c_float, aOffsetY: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetVisualViewportOffsetRelativeToLayoutViewport)(self, aOffsetX, aOffsetY)
    }


    /// ```text
    /// /**
    ///    * Returns the scroll position of the window's visual viewport.
    ///    */
    /// ```
    ///

    /// `void getVisualViewportOffset (out long aOffsetX, out long aOffsetY);`
    #[inline]
    pub unsafe fn GetVisualViewportOffset(&self, aOffsetX: *mut i32, aOffsetY: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetVisualViewportOffset)(self, aOffsetX, aOffsetY)
    }


    /// ```text
    /// /**
    ///    * Transforms the passed in rect from layout relative coords (relative to
        ///    * this document) to be is visual coords.
    ///    */
    /// ```
    ///

    /// `DOMRect transformRectLayoutToVisual (in float aX, in float aY, in float aWidth, in float aHeight);`
    #[inline]
    pub unsafe fn TransformRectLayoutToVisual(&self, aX: libc::c_float, aY: libc::c_float, aWidth: libc::c_float, aHeight: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).TransformRectLayoutToVisual)(self, aX, aY, aWidth, aHeight, _retval)
    }


    /// ```text
    /// /**
    ///    * Transform a rectangle given in coordinates relative to this document
    ///    * into CSS coordinates relative to the screen.
    ///    */
    /// ```
    ///

    /// `DOMRect toScreenRect (in float aX, in float aY, in float aWidth, in float aHeight);`
    #[inline]
    pub unsafe fn ToScreenRect(&self, aX: libc::c_float, aY: libc::c_float, aWidth: libc::c_float, aHeight: libc::c_float, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ToScreenRect)(self, aX, aY, aWidth, aHeight, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets the maximum height of the dynamic toolbar in Screen pixel units.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void setDynamicToolbarMaxHeight (in uint32_t aHeightInScreen);`
    #[inline]
    pub unsafe fn SetDynamicToolbarMaxHeight(&self, aHeightInScreen: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetDynamicToolbarMaxHeight)(self, aHeightInScreen)
    }


    /// ```text
    /// /**
    ///    * Returns true if a flush of the given type is needed.
    ///    */
    /// ```
    ///

    /// `bool needsFlush (in long aFlushtype);`
    #[inline]
    pub unsafe fn NeedsFlush(&self, aFlushtype: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).NeedsFlush)(self, aFlushtype, _retval)
    }


    /// ```text
    /// /**
    ///    * Flush pending layout-type notification without flushing throttled
    ///    * animations.
    ///    */
    /// ```
    ///

    /// `void flushLayoutWithoutThrottledAnimations ();`
    #[inline]
    pub unsafe fn FlushLayoutWithoutThrottledAnimations(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FlushLayoutWithoutThrottledAnimations)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns the bounds of the window's currently loaded document. This will
    ///    * generally be (0, 0, pageWidth, pageHeight) but in some cases (e.g. RTL
        ///    * documents) may have a negative left value.
    ///    */
    /// ```
    ///

    /// `DOMRect getRootBounds ();`
    #[inline]
    pub unsafe fn GetRootBounds(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetRootBounds)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Get IME open state. TRUE means 'Open', otherwise, 'Close'.
    ///    * This property works only when IMEEnabled is IME_STATUS_ENABLED.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean IMEIsOpen;`
    #[inline]
    pub unsafe fn GetIMEIsOpen(&self, aIMEIsOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIMEIsOpen)(self, aIMEIsOpen)
    }


    /// ```text
    /// /**
    ///    * Get IME status, see above IME_STATUS_* definitions.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long IMEStatus;`
    #[inline]
    pub unsafe fn GetIMEStatus(&self, aIMEStatus: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIMEStatus)(self, aIMEStatus)
    }


    /// ```text
    /// /**
    ///    * Get the number of screen pixels per CSS pixel.
    ///    *
    ///    * This is the same as window.devicePixelRatio, except it bypasses
    ///    * the fingerprinting resistance efforts that window.devicePixelRatio
    ///    * does (which is fine as this is only exposed to browser internals).
    ///    */
    /// ```
    ///

    /// `readonly attribute float screenPixelsPerCSSPixel;`
    #[inline]
    pub unsafe fn GetScreenPixelsPerCSSPixel(&self, aScreenPixelsPerCSSPixel: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetScreenPixelsPerCSSPixel)(self, aScreenPixelsPerCSSPixel)
    }


    /// ```text
    /// /**
    ///    * Get the number of screen pixels per CSS pixel,
    ///    * not taking into account any override of the device pixel scale
    ///    * that's imposed by Responsive Design Mode.
    ///    *
    ///    * This is needed to e.g. correctly translate "window.mozInnerScreenX/Y"
    ///    * into device pixels even when "window" is the window of a document
    ///    * inside an RDM pane.
    ///    */
    /// ```
    ///

    /// `readonly attribute float screenPixelsPerCSSPixelNoOverride;`
    #[inline]
    pub unsafe fn GetScreenPixelsPerCSSPixelNoOverride(&self, aScreenPixelsPerCSSPixelNoOverride: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetScreenPixelsPerCSSPixelNoOverride)(self, aScreenPixelsPerCSSPixelNoOverride)
    }


    /// ```text
    /// /**
    ///    * Get the current zoom factor.
    ///    * This is _approximately_ the same as nsIContentViewer.fullZoom,
    ///    * but takes into account Gecko's quantization of the zoom factor, which is
    ///    * implemented by adjusting the (integer) number of appUnits per devPixel.
    ///    */
    /// ```
    ///

    /// `readonly attribute float fullZoom;`
    #[inline]
    pub unsafe fn GetFullZoom(&self, aFullZoom: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetFullZoom)(self, aFullZoom)
    }


    /// ```text
    /// /**
    ///    * Dispatches aEvent as a synthesized trusted event for tests via the
    ///    * PresShell object of the window's document.
    ///    * The event is dispatched to aTarget, which should be an object
    ///    * which implements nsIContent interface (#element, #text, etc).
    ///    *
    ///    * Cannot be accessed from unprivileged context (not
        ///    * content-accessible) Will throw a DOM security error if called
    ///    * without chrome privileges.
    ///    *
    ///    * @note Event handlers won't get aEvent as parameter, but a similar event.
    ///    *       Also, aEvent should not be reused.
    ///    */
    /// ```
    ///

    /// `[can_run_script] boolean dispatchDOMEventViaPresShellForTesting (in Node aTarget, in Event aEvent);`
    #[inline]
    pub unsafe fn DispatchDOMEventViaPresShellForTesting(&self, aTarget: *const libc::c_void, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DispatchDOMEventViaPresShellForTesting)(self, aTarget, aEvent, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets WidgetEvent::mFlags::mOnlyChromeDispatch to true to ensure that
    ///    * the event is propagated only to chrome.
    ///    * Event's .target property will be aTarget.
    ///    * Returns the same value as what EventTarget.dispatchEvent does.
    ///    */
    /// ```
    ///

    /// `boolean dispatchEventToChromeOnly (in EventTarget aTarget, in Event aEvent);`
    #[inline]
    pub unsafe fn DispatchEventToChromeOnly(&self, aTarget: *const libc::c_void, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DispatchEventToChromeOnly)(self, aTarget, aEvent, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the real classname (possibly of the mostly-transparent security
        ///    * wrapper) of aObj.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] string getClassName (in jsval aObject);`
    const _GetClassName: () = ();

    /// ```text
    /// /**
    ///    * Generate a content command event.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible)
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    * @param aType Type of command content event to send.  Can be one of "cut",
    ///    *        "copy", "paste", "delete", "undo", "redo", or "pasteTransferable".
    ///    * @param aTransferable an instance of nsITransferable when aType is
    ///    *        "pasteTransferable"
    ///    */
    /// ```
    ///

    /// `void sendContentCommandEvent (in AString aType, [optional] in nsITransferable aTransferable);`
    #[inline]
    pub unsafe fn SendContentCommandEvent(&self, aType: *const ::nsstring::nsAString, aTransferable: *const nsITransferable) -> ::nserror::nsresult {
        ((*self.vtable).SendContentCommandEvent)(self, aType, aTransferable)
    }


    /// ```text
    /// /**
    ///    * Synthesize a query content event. Note that the result value returned here
    ///    * is in LayoutDevice pixels rather than CSS pixels.
    ///    *
    ///    * @param aType  One of the following const values.  And see also each comment
    ///    *               for the other parameters and the result.
    ///    * @param aAdditionalFlags See the description of QUERY_CONTENT_FLAG_*.
    ///    */
    /// ```
    ///

    /// `nsIQueryContentEventResult sendQueryContentEvent (in unsigned long aType, in long long aOffset, in unsigned long aLength, in long aX, in long aY, [optional] in unsigned long aAdditionalFlags);`
    #[inline]
    pub unsafe fn SendQueryContentEvent(&self, aType: u32, aOffset: i64, aLength: u32, aX: i32, aY: i32, aAdditionalFlags: u32, _retval: *mut*const nsIQueryContentEventResult) -> ::nserror::nsresult {
        ((*self.vtable).SendQueryContentEvent)(self, aType, aOffset, aLength, aX, aY, aAdditionalFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Called when the remote child frame has changed its fullscreen state,
    ///    * when entering fullscreen, and when the origin which is fullscreen changes.
    ///    * aFrameElement is the iframe element which contains the child-process
    ///    * fullscreen document.
    ///    */
    /// ```
    ///

    /// `void remoteFrameFullscreenChanged (in Element aFrameElement);`
    #[inline]
    pub unsafe fn RemoteFrameFullscreenChanged(&self, aFrameElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).RemoteFrameFullscreenChanged)(self, aFrameElement)
    }


    /// ```text
    /// /**
    ///    * Called when the remote frame has popped all fullscreen elements off its
    ///    * stack, so that the operation can complete on the parent side.
    ///    */
    /// ```
    ///

    /// `void remoteFrameFullscreenReverted ();`
    #[inline]
    pub unsafe fn RemoteFrameFullscreenReverted(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoteFrameFullscreenReverted)(self, )
    }


    /// ```text
    /// /**
    ///    * Calls the document to handle any pending fullscreen requests.
    ///    * It is called when the parent document has entered fullscreen, and
    ///    * we want to put the current document into fullscreen as well.
    ///    * The return value indicates whether there is any fullscreen request
    ///    * handled by this call.
    ///    */
    /// ```
    ///

    /// `boolean handleFullscreenRequests ();`
    #[inline]
    pub unsafe fn HandleFullscreenRequests(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleFullscreenRequests)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Called when the child frame has fully exit fullscreen, so that the parent
    ///    * process can also fully exit.
    ///    */
    /// ```
    ///

    /// `void exitFullscreen ();`
    #[inline]
    pub unsafe fn ExitFullscreen(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ExitFullscreen)(self, )
    }


    /// ```text
    /// /**
    ///    * Synthesize a selection set event to the window.
    ///    *
    ///    * This sets the selection as the specified information.
    ///    *
    ///    * @param aOffset  The caret offset of the selection start.
    ///    * @param aLength  The length of the selection.  If this is too long, the
    ///    *                 extra length is ignored.
    ///    * @param aAdditionalFlags See the description of SELECTION_SET_FLAG_*.
    ///    * @return True, if succeeded.  Otherwise, false.
    ///    */
    /// ```
    ///

    /// `boolean sendSelectionSetEvent (in unsigned long aOffset, in unsigned long aLength, [optional] in unsigned long aAdditionalFlags);`
    #[inline]
    pub unsafe fn SendSelectionSetEvent(&self, aOffset: u32, aLength: u32, aAdditionalFlags: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SendSelectionSetEvent)(self, aOffset, aLength, aAdditionalFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Select content at a client point based on a selection behavior if the
    ///    * underlying content is selectable. Selection will accumulate with any
    ///    * existing selection, callers should clear selection prior if needed.
    ///    * May fire selection changed events. Calls nsFrame's SelectByTypeAtPoint.
    ///    *
    ///    * @param aX, aY The selection point in client coordinates.
    ///    * @param aSelectType The selection behavior requested.
    ///    * @return True if a selection occured, false otherwise.
    ///    * @throw NS_ERROR_DOM_SECURITY_ERR, NS_ERROR_UNEXPECTED for utils
    ///    * issues, and NS_ERROR_INVALID_ARG for coordinates that are outside
    ///    * this window.
    ///    */
    /// ```
    ///

    /// `[can_run_script] boolean selectAtPoint (in float aX, in float aY, in unsigned long aSelectBehavior);`
    #[inline]
    pub unsafe fn SelectAtPoint(&self, aX: libc::c_float, aY: libc::c_float, aSelectBehavior: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SelectAtPoint)(self, aX, aY, aSelectBehavior, _retval)
    }


    /// ```text
    /// /**
    ///    * Perform the equivalent of:
    ///    *   window.getComputedStyle(aElement, aPseudoElement).
    ///    *     getPropertyValue(aPropertyName)
    ///    * except that, when the link whose presence in history is allowed to
    ///    * influence aElement's style is visited, get the value the property
    ///    * would have if allowed all properties to change as a result of
    ///    * :visited selectors (except for cases where getComputedStyle uses
        ///    * data from the frame).
    ///    *
    ///    * This is easier to implement than adding our property restrictions
    ///    * to this API, and is sufficient for the present testing
    ///    * requirements (which are essentially testing 'color').
    ///    */
    /// ```
    ///

    /// `AString getVisitedDependentComputedStyle (in Element aElement, in AString aPseudoElement, in AString aPropertyName);`
    #[inline]
    pub unsafe fn GetVisitedDependentComputedStyle(&self, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, aPropertyName: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVisitedDependentComputedStyle)(self, aElement, aPseudoElement, aPropertyName, _retval)
    }


    /// ```text
    /// /**
    ///    * DEPRECATED. Use WindowGlobalChild.outerWindowId instead.
    ///    *
    ///    * Get the id of the outer window of this window.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long deprecatedOuterWindowID;`
    #[inline]
    pub unsafe fn GetDeprecatedOuterWindowID(&self, aDeprecatedOuterWindowID: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetDeprecatedOuterWindowID)(self, aDeprecatedOuterWindowID)
    }


    /// ```text
    /// /**
    ///    * Put the window into a state where scripts are frozen and events
    ///    * suppressed, for use when the window has launched a modal prompt.
    ///    */
    /// ```
    ///

    /// `void enterModalState ();`
    #[inline]
    pub unsafe fn EnterModalState(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnterModalState)(self, )
    }


    /// ```text
    /// /**
    ///    * Resume normal window state, where scripts can run and events are
    ///    * delivered.
    ///    */
    /// ```
    ///

    /// `void leaveModalState ();`
    #[inline]
    pub unsafe fn LeaveModalState(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LeaveModalState)(self, )
    }


    /// ```text
    /// /**
    ///    * Is the window is in a modal state? [See enterModalState()]
    ///    */
    /// ```
    ///

    /// `boolean isInModalState ();`
    #[inline]
    pub unsafe fn IsInModalState(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsInModalState)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Internal desktopMode flag.
    ///    */
    /// ```
    ///

    /// `attribute boolean desktopModeViewport;`
    #[inline]
    pub unsafe fn GetDesktopModeViewport(&self, aDesktopModeViewport: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDesktopModeViewport)(self, aDesktopModeViewport)
    }


    /// ```text
    /// /**
    ///    * Internal desktopMode flag.
    ///    */
    /// ```
    ///

    /// `attribute boolean desktopModeViewport;`
    #[inline]
    pub unsafe fn SetDesktopModeViewport(&self, aDesktopModeViewport: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDesktopModeViewport)(self, aDesktopModeViewport)
    }


    /// ```text
    /// /**
    ///    * Suspend/resume timeouts on this window and its descendant windows.
    ///    */
    /// ```
    ///

    /// `void suspendTimeouts ();`
    #[inline]
    pub unsafe fn SuspendTimeouts(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SuspendTimeouts)(self, )
    }



    /// `void resumeTimeouts ();`
    #[inline]
    pub unsafe fn ResumeTimeouts(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumeTimeouts)(self, )
    }


    /// ```text
    /// /**
    ///    * What type of layer manager the widget associated with this window is
    ///    * using. "Basic" is unaccelerated; other types are accelerated. Throws an
    ///    * error if there is no widget associated with this window.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString layerManagerType;`
    #[inline]
    pub unsafe fn GetLayerManagerType(&self, aLayerManagerType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLayerManagerType)(self, aLayerManagerType)
    }


    /// ```text
    /// /**
    ///    * True if the layer manager for the widget associated with this window is
    ///    * forwarding layers to a remote compositor, false otherwise. Throws an
    ///    * error if there is no widget associated with this window.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean layerManagerRemote;`
    #[inline]
    pub unsafe fn GetLayerManagerRemote(&self, aLayerManagerRemote: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLayerManagerRemote)(self, aLayerManagerRemote)
    }


    /// ```text
    /// /**
    ///    * True if advanced layers is enabled on this window, false otherwise.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean usingAdvancedLayers;`
    #[inline]
    pub unsafe fn GetUsingAdvancedLayers(&self, aUsingAdvancedLayers: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUsingAdvancedLayers)(self, aUsingAdvancedLayers)
    }


    /// ```text
    /// /**
    ///    * True if webrender was requested by the user (via pref or env-var), false
    ///    * otherwise. Note that this doesn't represent whether or not webrender is
    ///    * *actually* enabled, just whether or not it was requested.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isWebRenderRequested;`
    #[inline]
    pub unsafe fn GetIsWebRenderRequested(&self, aIsWebRenderRequested: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsWebRenderRequested)(self, aIsWebRenderRequested)
    }


    /// ```text
    /// /**
    ///    * Returns the current audio backend as a free-form string.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString currentAudioBackend;`
    #[inline]
    pub unsafe fn GetCurrentAudioBackend(&self, aCurrentAudioBackend: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentAudioBackend)(self, aCurrentAudioBackend)
    }


    /// ```text
    /// /**
    ///    * Returns the max channel counts of the current audio device.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long currentMaxAudioChannels;`
    #[inline]
    pub unsafe fn GetCurrentMaxAudioChannels(&self, aCurrentMaxAudioChannels: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentMaxAudioChannels)(self, aCurrentMaxAudioChannels)
    }


    /// ```text
    /// /**
    ///    * Returns the mean round trip latency in seconds for the default input and
    ///    * output device, and the stddev of this latency, as a two element array when
    ///    * the Promise succeeds.
    ///    */
    /// ```
    ///

    /// `Promise defaultDevicesRoundTripLatency ();`
    const _DefaultDevicesRoundTripLatency: () = ();

    /// ```text
    /// /**
    ///    * Returns the preferred sample rate of the current audio device.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long currentPreferredSampleRate;`
    #[inline]
    pub unsafe fn GetCurrentPreferredSampleRate(&self, aCurrentPreferredSampleRate: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentPreferredSampleRate)(self, aCurrentPreferredSampleRate)
    }



    /// `nsIArray audioDevices (in unsigned short aSide);`
    #[inline]
    pub unsafe fn AudioDevices(&self, aSide: u16, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).AudioDevices)(self, aSide, _retval)
    }


    /// ```text
    /// /**
    ///    * Record (and return) frame-intervals for frames which were presented
    ///    *   between calling StartFrameTimeRecording and StopFrameTimeRecording.
    ///    *
    ///    * - Uses a cyclic buffer and serves concurrent consumers, so if Stop is called too late
    ///    *     (elements were overwritten since Start), result is considered invalid and hence empty.
    ///    * - Buffer is capable of holding 10 seconds @ 60fps (or more if frames were less frequent).
    ///    *     Can be changed (up to 1 hour) via pref: toolkit.framesRecording.bufferSize.
    ///    * - Note: the first frame-interval may be longer than expected because last frame
    ///    *     might have been presented some time before calling StartFrameTimeRecording.
    ///    */
    /// /**
    ///    * Returns a handle which represents current recording start position.
    ///    */
    /// ```
    ///

    /// `void startFrameTimeRecording ([retval] out unsigned long startIndex);`
    #[inline]
    pub unsafe fn StartFrameTimeRecording(&self, startIndex: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).StartFrameTimeRecording)(self, startIndex)
    }


    /// ```text
    /// /**
    ///    * Returns array of frame intervals since the time when the given startIndex
    ///    * was handed out from startFrameTimeRecording.
    ///    */
    /// ```
    ///

    /// `Array<float> stopFrameTimeRecording (in unsigned long startIndex);`
    #[inline]
    pub unsafe fn StopFrameTimeRecording(&self, startIndex: u32, _retval: *mut thin_vec::ThinVec<libc::c_float>) -> ::nserror::nsresult {
        ((*self.vtable).StopFrameTimeRecording)(self, startIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * The DPI of the display
    ///    */
    /// ```
    ///

    /// `readonly attribute float displayDPI;`
    #[inline]
    pub unsafe fn GetDisplayDPI(&self, aDisplayDPI: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayDPI)(self, aDisplayDPI)
    }


    /// ```text
    /// /**
    ///    * advanceTimeAndRefresh allows the caller to take over the refresh
    ///    * driver timing for a window.  A call to advanceTimeAndRefresh does
    ///    * three things:
    ///    *  (1) It marks the refresh driver for this presentation so that it
    ///    *      no longer refreshes on its own, but is instead driven entirely
    ///    *      by the caller (except for the refresh that happens when a
        ///    *      document comes out of the bfcache).
    ///    *  (2) It advances the refresh driver's current refresh time by the
    ///    *      argument given.  Negative advances are permitted.
    ///    *  (3) It does a refresh (i.e., notifies refresh observers) at that
    ///    *      new time.
    ///    *
    ///    * Note that this affects other connected docshells of the same type
    ///    * in the same docshell tree, such as parent frames.
    ///    *
    ///    * When callers have completed their use of advanceTimeAndRefresh,
    ///    * they must call restoreNormalRefresh.
    ///    */
    /// ```
    ///

    /// `void advanceTimeAndRefresh (in long long aMilliseconds);`
    #[inline]
    pub unsafe fn AdvanceTimeAndRefresh(&self, aMilliseconds: i64) -> ::nserror::nsresult {
        ((*self.vtable).AdvanceTimeAndRefresh)(self, aMilliseconds)
    }


    /// ```text
    /// /**
    ///    * Undoes the effects of advanceTimeAndRefresh.
    ///    */
    /// ```
    ///

    /// `void restoreNormalRefresh ();`
    #[inline]
    pub unsafe fn RestoreNormalRefresh(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RestoreNormalRefresh)(self, )
    }


    /// ```text
    /// /**
    ///    * Reports whether the current state is test-controlled refreshes
    ///    * (see advanceTimeAndRefresh and restoreNormalRefresh above).
    ///    */
    /// ```
    ///

    /// `readonly attribute bool isTestControllingRefreshes;`
    #[inline]
    pub unsafe fn GetIsTestControllingRefreshes(&self, aIsTestControllingRefreshes: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTestControllingRefreshes)(self, aIsTestControllingRefreshes)
    }


    /// ```text
    /// /**
    ///    * Reports whether APZ is enabled on the widget that this window is attached
    ///    * to. If there is no widget it will report the default platform value of
    ///    * whether or not APZ is enabled.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool asyncPanZoomEnabled;`
    #[inline]
    pub unsafe fn GetAsyncPanZoomEnabled(&self, aAsyncPanZoomEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAsyncPanZoomEnabled)(self, aAsyncPanZoomEnabled)
    }


    /// ```text
    /// /**
    ///    * Set async scroll offset on an element. The next composite will render
    ///    * with that offset if async scrolling is enabled, and then the offset
    ///    * will be removed. Only call this while test-controlled refreshes is enabled.
    ///    */
    /// ```
    ///

    /// `void setAsyncScrollOffset (in Element aElement, in float aX, in float aY);`
    #[inline]
    pub unsafe fn SetAsyncScrollOffset(&self, aElement: *const libc::c_void, aX: libc::c_float, aY: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetAsyncScrollOffset)(self, aElement, aX, aY)
    }


    /// ```text
    /// /**
    ///    * Set async zoom value. aRootElement should be the document element of our
    ///    * document. The next composite will render with that zoom added to any
    ///    * existing zoom if async scrolling is enabled, and then the zoom will be
    ///    * removed. Only call this while test-controlled refreshes is enabled.
    ///    */
    /// ```
    ///

    /// `void setAsyncZoom (in Element aRootElement, in float aValue);`
    #[inline]
    pub unsafe fn SetAsyncZoom(&self, aRootElement: *const libc::c_void, aValue: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetAsyncZoom)(self, aRootElement, aValue)
    }


    /// ```text
    /// /**
    ///    * Do a round-trip to the compositor to ensure any pending APZ repaint requests
    ///    * get flushed to the main thread. If the function returns true, the flush was
    ///    * triggered and an "apz-repaints-flushed" notification will be dispatched via
    ///    * the observer service once the flush is complete. If the function returns
    ///    * false, an error occurred or a flush is not needed, and the notification
    ///    * will not fire. This is intended to be used by test code only!
    ///    */
    /// ```
    ///

    /// `bool flushApzRepaints ();`
    #[inline]
    pub unsafe fn FlushApzRepaints(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).FlushApzRepaints)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets a flag on the element to forcibly disable APZ on it. This affects
    ///    * the result of nsLayoutUtils::ShouldDisableApzForElement when called on
    ///    * this element. This function also schedules a repaint to ensure that the
    ///    * change takes effect. Note that this is not reversible; it is intended for
    ///    * use by test code only.
    ///    */
    /// ```
    ///

    /// `void disableApzForElement (in Element aElement);`
    #[inline]
    pub unsafe fn DisableApzForElement(&self, aElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).DisableApzForElement)(self, aElement)
    }


    /// ```text
    /// /**
    ///    * Ask APZ to pan and zoom to the focused input element.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void zoomToFocusedInput ();`
    #[inline]
    pub unsafe fn ZoomToFocusedInput(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ZoomToFocusedInput)(self, )
    }


    /// ```text
    /// /**
    ///    * Method for testing StyleAnimationValue::ComputeDistance.
    ///    *
    ///    * Returns the distance between the two values as reported by
    ///    * StyleAnimationValue::ComputeDistance for the given element and
    ///    * property.
    ///    */
    /// ```
    ///

    /// `double computeAnimationDistance (in Element element, in AString property, in AString value1, in AString value2);`
    #[inline]
    pub unsafe fn ComputeAnimationDistance(&self, element: *const libc::c_void, property: *const ::nsstring::nsAString, value1: *const ::nsstring::nsAString, value2: *const ::nsstring::nsAString, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).ComputeAnimationDistance)(self, element, property, value1, value2, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the computed style for the specified property of given pseudo type
    ///    * on the given element after removing styles from declarative animations.
    ///    * @param aElement - A target element
    ///    * @param aPseudoElement - A pseudo type (e.g. '::before' or null)
    ///    * @param aProperty - A longhand CSS property (e.g. 'background-color')
    ///    * @param aFlushType - FLUSH_NONE if any pending styles should not happen,
    ///    *                     FLUSH_STYLE to flush pending styles.
    ///    */
    /// ```
    ///

    /// `AString getUnanimatedComputedStyle (in Element aElement, in AString aPseudoElement, in AString aProperty, in long aFlushType);`
    #[inline]
    pub unsafe fn GetUnanimatedComputedStyle(&self, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, aProperty: *const ::nsstring::nsAString, aFlushType: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUnanimatedComputedStyle)(self, aElement, aPseudoElement, aProperty, aFlushType, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the type of the currently focused html input, if any.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString focusedInputType;`
    #[inline]
    pub unsafe fn GetFocusedInputType(&self, aFocusedInputType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedInputType)(self, aFocusedInputType)
    }


    /// ```text
    /// /**
    ///    * Get the action hint of the currently focused html input, if any.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString focusedActionHint;`
    #[inline]
    pub unsafe fn GetFocusedActionHint(&self, aFocusedActionHint: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedActionHint)(self, aFocusedActionHint)
    }


    /// ```text
    /// /**
    ///    * Get the inputmode of the currently focused editing host, if any.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString focusedInputMode;`
    #[inline]
    pub unsafe fn GetFocusedInputMode(&self, aFocusedInputMode: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedInputMode)(self, aFocusedInputMode)
    }


    /// ```text
    /// /**
    ///    * Get the autocapitalize of the currently focused editing host, if any.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString focusedAutocapitalize;`
    #[inline]
    pub unsafe fn GetFocusedAutocapitalize(&self, aFocusedAutocapitalize: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedAutocapitalize)(self, aFocusedAutocapitalize)
    }


    /// ```text
    /// /**
    ///    * Find the view ID for a given element. This is the reverse of
    ///    * findElementWithViewId().
    ///    */
    /// ```
    ///

    /// `nsViewID getViewId (in Element aElement);`
    #[inline]
    pub unsafe fn GetViewId(&self, aElement: *const libc::c_void, _retval: *mut nsViewID) -> ::nserror::nsresult {
        ((*self.vtable).GetViewId)(self, aElement, _retval)
    }


    /// ```text
    /// /**
    ///    * Checks the layer tree for this window and returns true
    ///    * if all layers have transforms that are translations by integers,
    ///    * no leaf layers overlap, and the union of the leaf layers is exactly
    ///    * the bounds of the window. Always returns true in non-DEBUG builds.
    ///    */
    /// ```
    ///

    /// `boolean leafLayersPartitionWindow ();`
    #[inline]
    pub unsafe fn LeafLayersPartitionWindow(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).LeafLayersPartitionWindow)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if any PaintedLayer painting has been done for this element,
    ///    * clears the painted flags if they have.
    ///    */
    /// ```
    ///

    /// `boolean checkAndClearPaintedState (in Element aElement);`
    #[inline]
    pub unsafe fn CheckAndClearPaintedState(&self, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckAndClearPaintedState)(self, aElement, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if any display list building has been done for this element,
    ///    * clears the display list flags if they have.
    ///    */
    /// ```
    ///

    /// `boolean checkAndClearDisplayListState (in Element aElement);`
    #[inline]
    pub unsafe fn CheckAndClearDisplayListState(&self, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckAndClearDisplayListState)(self, aElement, _retval)
    }


    /// ```text
    /// /**
    ///    * Check whether all display items of the primary frame of aElement have been
    ///    * assigned to the same single PaintedLayer in the last paint. If that is the
    ///    * case, returns whether that PaintedLayer is opaque; if it's not the case, an
    ///    * exception is thrown.
    ///    */
    /// ```
    ///

    /// `boolean isPartOfOpaqueLayer (in Element aElement);`
    #[inline]
    pub unsafe fn IsPartOfOpaqueLayer(&self, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPartOfOpaqueLayer)(self, aElement, _retval)
    }


    /// ```text
    /// /**
    ///    * Count the number of different PaintedLayers that the supplied elements have
    ///    * been assigned to in the last paint. Throws an exception if any of the
    ///    * elements doesn't have a primary frame, or if that frame's display items are
    ///    * assigned to any other layers than just a single PaintedLayer per element.
    ///    */
    /// ```
    ///

    /// `unsigned long numberOfAssignedPaintedLayers (in Array<Element> aElements);`
    #[inline]
    pub unsafe fn NumberOfAssignedPaintedLayers(&self, aElements: *const thin_vec::ThinVec<*const libc::c_void>, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).NumberOfAssignedPaintedLayers)(self, aElements, _retval)
    }


    /// ```text
    /// /**
    ///    * Get internal id of the stored blob, file or file handle.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] long long getFileId (in jsval aFile);`
    const _GetFileId: () = ();

    /// ```text
    /// /**
    ///    * Get internal file path of the stored file or file handle.
    ///    *
    ///    * TODO: File handle objects are actually not supported at the moment.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] AString getFilePath (in jsval aFile);`
    const _GetFilePath: () = ();

    /// ```text
    /// /**
    ///    * Get file ref count info for given database and file id.
    ///    *
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] boolean getFileReferences (in AString aDatabaseName, in long long aId, [optional] in jsval aOptions, [optional] out long aRefCnt, [optional] out long aDBRefCnt);`
    const _GetFileReferences: () = ();


    /// `void flushPendingFileDeletions ();`
    #[inline]
    pub unsafe fn FlushPendingFileDeletions(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FlushPendingFileDeletions)(self, )
    }


    /// ```text
    /// /**
    ///    * Begin opcode-level profiling of all JavaScript execution in the window's
    ///    * runtime.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void startPCCountProfiling ();`
    const _StartPCCountProfiling: () = ();

    /// ```text
    /// /**
    ///    * Stop opcode-level profiling of JavaScript execution in the runtime, and
    ///    * collect all counts for use by getPCCount methods.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void stopPCCountProfiling ();`
    const _StopPCCountProfiling: () = ();

    /// ```text
    /// /**
    ///    * Purge collected PC counters.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void purgePCCounts ();`
    const _PurgePCCounts: () = ();

    /// ```text
    /// /**
    ///    * Get the number of scripts with opcode-level profiling information.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] long getPCCountScriptCount ();`
    const _GetPCCountScriptCount: () = ();

    /// ```text
    /// /**
    ///    * Get a JSON string for a short summary of a script and the PC counts
    ///    * accumulated for it.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] AString getPCCountScriptSummary (in long script);`
    const _GetPCCountScriptSummary: () = ();

    /// ```text
    /// /**
    ///    * Get a JSON string with full information about a profiled script,
    ///    * including the decompilation of the script and placement of decompiled
    ///    * operations within it, and PC counts for each operation.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] AString getPCCountScriptContents (in long script);`
    const _GetPCCountScriptContents: () = ();

    /// ```text
    /// /**
    ///    * Returns true if painting is suppressed for this window and false
    ///    * otherwise.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean paintingSuppressed;`
    #[inline]
    pub unsafe fn GetPaintingSuppressed(&self, aPaintingSuppressed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPaintingSuppressed)(self, aPaintingSuppressed)
    }


    /// ```text
    /// /**
    ///    * Returns an array of plugins on the page for opt-in activation.
    ///    *
    ///    * Cannot be accessed from unprivileged context (not content-accessible).
    ///    * Will throw a DOM security error if called without chrome privileges.
    ///    *
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval plugins;`
    const _GetPlugins: () = ();

    /// ```text
    /// /**
    ///    * Set the viewport size for the purposes of clamping scroll positions for
    ///    * the root scroll frame of this document to be (aWidth,aHeight) in CSS pixels.
    ///    *
    ///    * The caller of this method must have chrome privileges.
    ///    */
    /// ```
    ///

    /// `void setVisualViewportSize (in float aWidth, in float aHeight);`
    #[inline]
    pub unsafe fn SetVisualViewportSize(&self, aWidth: libc::c_float, aHeight: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).SetVisualViewportSize)(self, aWidth, aHeight)
    }


    /// ```text
    /// /**
    ///    * These are used to control whether dialogs (alert, prompt, confirm) are
    ///    * allowed.
    ///    */
    /// ```
    ///

    /// `void disableDialogs ();`
    #[inline]
    pub unsafe fn DisableDialogs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DisableDialogs)(self, )
    }



    /// `void enableDialogs ();`
    #[inline]
    pub unsafe fn EnableDialogs(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableDialogs)(self, )
    }



    /// `bool areDialogsEnabled ();`
    #[inline]
    pub unsafe fn AreDialogsEnabled(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AreDialogsEnabled)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Synchronously loads a style sheet from |sheetURI| and adds it to the list
    ///    * of additional style sheets of the document.
    ///    *
    ///    * These additional style sheets are very much like user/agent sheets loaded
    ///    * with loadAndRegisterSheet. The only difference is that they are applied only
    ///    * on the document owned by this window.
    ///    *
    ///    * Sheets added via this API take effect immediately on the document.
    ///    */
    /// ```
    ///

    /// `void loadSheet (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn LoadSheet(&self, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).LoadSheet)(self, sheetURI, type_)
    }


    /// ```text
    /// /**
    ///    * Same as the above method but allows passing the URI as a string.
    ///    */
    /// ```
    ///

    /// `void loadSheetUsingURIString (in ACString sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn LoadSheetUsingURIString(&self, sheetURI: *const ::nsstring::nsACString, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).LoadSheetUsingURIString)(self, sheetURI, type_)
    }


    /// ```text
    /// /**
    ///    * Adds a style sheet to the list of additional style sheets of the document.
    ///    *
    ///    * Style sheets can be preloaded with nsIStyleSheetService.preloadSheet.
    ///    *
    ///    * Sheets added via this API take effect immediately on the document.
    ///    */
    /// ```
    ///

    /// `void addSheet (in nsIPreloadedStyleSheet sheet, in unsigned long type);`
    #[inline]
    pub unsafe fn AddSheet(&self, sheet: *const nsIPreloadedStyleSheet, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).AddSheet)(self, sheet, type_)
    }


    /// ```text
    /// /**
    ///    * Remove the document style sheet at |sheetURI| from the list of additional
    ///    * style sheets of the document.  The removal takes effect immediately.
    ///    */
    /// ```
    ///

    /// `void removeSheet (in nsIURI sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn RemoveSheet(&self, sheetURI: *const nsIURI, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveSheet)(self, sheetURI, type_)
    }


    /// ```text
    /// /**
    ///    * Same as the above method but allows passing the URI as a string.
    ///    */
    /// ```
    ///

    /// `void removeSheetUsingURIString (in ACString sheetURI, in unsigned long type);`
    #[inline]
    pub unsafe fn RemoveSheetUsingURIString(&self, sheetURI: *const ::nsstring::nsACString, type_: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveSheetUsingURIString)(self, sheetURI, type_)
    }


    /// ```text
    /// /**
    ///    * Returns true if a user input is being handled.
    ///    *
    ///    * This calls EventStateManager::IsHandlingUserInput().
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isHandlingUserInput;`
    #[inline]
    pub unsafe fn GetIsHandlingUserInput(&self, aIsHandlingUserInput: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsHandlingUserInput)(self, aIsHandlingUserInput)
    }


    /// ```text
    /// /**
    ///    * Returns milliseconds elapsed since last user input was started.
    ///    * Returns -1 if there wasn't any previous user input.
    ///    *
    ///    * This relies on EventStateManager::LatestUserInputStart()
    ///    */
    /// ```
    ///

    /// `readonly attribute double millisSinceLastUserInput;`
    #[inline]
    pub unsafe fn GetMillisSinceLastUserInput(&self, aMillisSinceLastUserInput: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetMillisSinceLastUserInput)(self, aMillisSinceLastUserInput)
    }


    /// ```text
    /// /**
    ///    * After calling the method, the window for which this DOMWindowUtils
    ///    * was created can be closed using scripts.
    ///    */
    /// ```
    ///

    /// `void allowScriptsToClose ();`
    #[inline]
    pub unsafe fn AllowScriptsToClose(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AllowScriptsToClose)(self, )
    }


    /// ```text
    /// /**
    ///    * Is the parent window's main widget visible?  If it isn't, we probably
    ///    * don't want to display any dialogs etc it may request.  This corresponds
    ///    * to the visibility check in nsWindowWatcher::OpenWindowInternal().
    ///    *
    ///    * Will throw a DOM security error if called without chrome privileges or
    ///    * NS_ERROR_NOT_AVAILABLE in the unlikely event that the parent window's
    ///    * main widget can't be reached.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isParentWindowMainWidgetVisible;`
    #[inline]
    pub unsafe fn GetIsParentWindowMainWidgetVisible(&self, aIsParentWindowMainWidgetVisible: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsParentWindowMainWidgetVisible)(self, aIsParentWindowMainWidgetVisible)
    }


    /// ```text
    /// /**
    ///    * In certain cases the event handling of nodes, form controls in practice,
    ///    * may be disabled. Such cases are for example the existence of disabled
    ///    * attribute or -moz-user-input: none.
    ///    */
    /// ```
    ///

    /// `boolean isNodeDisabledForEvents (in Node aNode);`
    #[inline]
    pub unsafe fn IsNodeDisabledForEvents(&self, aNode: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsNodeDisabledForEvents)(self, aNode, _retval)
    }


    /// ```text
    /// /**
    ///    * Setting paintFlashing to true will flash newly painted area.
    ///    */
    /// ```
    ///

    /// `attribute boolean paintFlashing;`
    #[inline]
    pub unsafe fn GetPaintFlashing(&self, aPaintFlashing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPaintFlashing)(self, aPaintFlashing)
    }


    /// ```text
    /// /**
    ///    * Setting paintFlashing to true will flash newly painted area.
    ///    */
    /// ```
    ///

    /// `attribute boolean paintFlashing;`
    #[inline]
    pub unsafe fn SetPaintFlashing(&self, aPaintFlashing: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPaintFlashing)(self, aPaintFlashing)
    }



    /// `AString getOMTAStyle (in Element aElement, in AString aProperty, [optional] in AString aPseudoElement);`
    #[inline]
    pub unsafe fn GetOMTAStyle(&self, aElement: *const libc::c_void, aProperty: *const ::nsstring::nsAString, aPseudoElement: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOMTAStyle)(self, aElement, aProperty, aPseudoElement, _retval)
    }



    /// `AString getOMTCTransform (in Element aElement, [optional] in AString aPseudoElement);`
    #[inline]
    pub unsafe fn GetOMTCTransform(&self, aElement: *const libc::c_void, aPseudoElement: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOMTCTransform)(self, aElement, aPseudoElement, _retval)
    }



    /// `bool isAnimationInPendingTracker (in Animation aAnimation);`
    #[inline]
    pub unsafe fn IsAnimationInPendingTracker(&self, aAnimation: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAnimationInPendingTracker)(self, aAnimation, _retval)
    }


    /// ```text
    /// /**
    ///    * If aHandlingInput is true, this informs the event state manager that
    ///    * we're handling user input. Otherwise, this is a no-op (as by default
        ///    * we're not handling user input).
    ///    * Remember to call destruct() on the return value!
    ///    * See also nsIDOMWindowUtils::isHandlingUserInput.
    ///    */
    /// ```
    ///

    /// `nsIJSRAIIHelper setHandlingUserInput (in boolean aHandlingInput);`
    #[inline]
    pub unsafe fn SetHandlingUserInput(&self, aHandlingInput: bool, _retval: *mut*const nsIJSRAIIHelper) -> ::nserror::nsresult {
        ((*self.vtable).SetHandlingUserInput)(self, aHandlingInput, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the content- and compositor-side APZ test data instances.
    ///    * The return values are of type APZTestData (see APZTestData.webidl).
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getContentAPZTestData ();`
    const _GetContentAPZTestData: () = ();


    /// `[implicit_jscontext] jsval getCompositorAPZTestData ();`
    const _GetCompositorAPZTestData: () = ();

    /// ```text
    /// /**
    ///    * Posts an RestyleHint::RESTYLE_SELF restyle event for the given element.
    ///    */
    /// ```
    ///

    /// `void postRestyleSelfEvent (in Element aElement);`
    #[inline]
    pub unsafe fn PostRestyleSelfEvent(&self, aElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).PostRestyleSelfEvent)(self, aElement)
    }


    /// ```text
    /// /**
    ///    * This method doesn't do anything useful.  It was solely added for the
    ///    * purpose of the test for bug 503926.
    ///    */
    /// ```
    ///

    /// `void xpconnectArgument (in nsISupports aObj);`
    #[inline]
    pub unsafe fn XpconnectArgument(&self, aObj: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).XpconnectArgument)(self, aObj)
    }


    /// ```text
    /// /**
    ///    * Helper for JS components that need to send permission requests with
    ///    * e10s support properly.
    ///    */
    /// ```
    ///

    /// `void askPermission (in nsIContentPermissionRequest aRequest);`
    #[inline]
    pub unsafe fn AskPermission(&self, aRequest: *const nsIContentPermissionRequest) -> ::nserror::nsresult {
        ((*self.vtable).AskPermission)(self, aRequest)
    }


    /// ```text
    /// /**
    ///    * Restyle generation for the current document.
    ///    *
    ///    * May throw NS_ERROR_NOT_AVAILABLE.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long restyleGeneration;`
    #[inline]
    pub unsafe fn GetRestyleGeneration(&self, aRestyleGeneration: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetRestyleGeneration)(self, aRestyleGeneration)
    }


    /// ```text
    /// /**
    ///    * Number of frames constructed (excluding breaking) for the curent
    ///    * document.
    ///    *
    ///    * May throw NS_ERROR_NOT_AVAILABLE.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long framesConstructed;`
    #[inline]
    pub unsafe fn GetFramesConstructed(&self, aFramesConstructed: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetFramesConstructed)(self, aFramesConstructed)
    }


    /// ```text
    /// /**
    ///    * Number of frames reflowed for the curent document.
    ///    *
    ///    * May throw NS_ERROR_NOT_AVAILABLE.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long framesReflowed;`
    #[inline]
    pub unsafe fn GetFramesReflowed(&self, aFramesReflowed: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetFramesReflowed)(self, aFramesReflowed)
    }


    /// ```text
    /// /**
    ///    * Controls the amount of chrome that should be visible on each side of
    ///    * the window. Works like the chromemargin xul:window attribute.
    ///    * This should only be used with non-XUL windows.
    ///    */
    /// ```
    ///

    /// `void setChromeMargin (in int32_t aTop, in int32_t aRight, in int32_t aBottom, in int32_t aLeft);`
    #[inline]
    pub unsafe fn SetChromeMargin(&self, aTop: int32_t, aRight: int32_t, aBottom: int32_t, aLeft: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetChromeMargin)(self, aTop, aRight, aBottom, aLeft)
    }


    /// ```text
    /// /**
    ///    * Returns a JSObject which contains a list of frame uniformities
    ///    * when the pref gfx.vsync.collect-scroll-data is enabled.
    ///    * Every result contains a layer address and a frame uniformity for that layer.
    ///    * A negative frame uniformity value indicates an invalid frame uniformity and an error has occured.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval getFrameUniformityTestData ();`
    const _GetFrameUniformityTestData: () = ();


    /// `void enterChaosMode ();`
    #[inline]
    pub unsafe fn EnterChaosMode(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnterChaosMode)(self, )
    }


    /// ```text
    /// /**
    ///    * Decrease the chaos mode activation level. See enterChaosMode().
    ///    */
    /// ```
    ///

    /// `void leaveChaosMode ();`
    #[inline]
    pub unsafe fn LeaveChaosMode(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LeaveChaosMode)(self, )
    }


    /// ```text
    /// /**
    ///    * Alerts Gecko of a device reset
    ///    */
    /// ```
    ///

    /// `void triggerDeviceReset ();`
    #[inline]
    pub unsafe fn TriggerDeviceReset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TriggerDeviceReset)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns whether the document's style set's rule processor for the
    ///    * specified level of the cascade is shared by multiple style sets.
    ///    * (Used by tests to ensure that certain optimizations do not regress.)
    ///    *
    ///    * @param aSheetType One of the nsIStyleSheetService.*_SHEET constants.
    ///    */
    /// ```
    ///

    /// `bool hasRuleProcessorUsedByMultipleStyleSets (in unsigned long aSheetType);`
    #[inline]
    pub unsafe fn HasRuleProcessorUsedByMultipleStyleSets(&self, aSheetType: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasRuleProcessorUsedByMultipleStyleSets)(self, aSheetType, _retval)
    }


    /// ```text
    /// /**
    ///    * Enable or disable displayport suppression. This is intended to be used by
    ///    * testing code, to provide more deterministic behaviour over the displayport
    ///    * suppression during tests. Note that this updates a flag, so whatever value
    ///    * was last provided is what will be used.
    ///    */
    /// ```
    ///

    /// `void respectDisplayPortSuppression (in boolean aEnabled);`
    #[inline]
    pub unsafe fn RespectDisplayPortSuppression(&self, aEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).RespectDisplayPortSuppression)(self, aEnabled)
    }


    /// ```text
    /// /**
    ///    * Set a flag that forces the next reflow interrupt check to return true. This
    ///    * can be used by tests to force execution of the interrupted reflow codepaths.
    ///    */
    /// ```
    ///

    /// `void forceReflowInterrupt ();`
    #[inline]
    pub unsafe fn ForceReflowInterrupt(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ForceReflowInterrupt)(self, )
    }


    /// ```text
    /// /**
    ///    * Terminate the GPU process. Used for testing GPU process restarts.
    ///    */
    /// ```
    ///

    /// `void terminateGPUProcess ();`
    #[inline]
    pub unsafe fn TerminateGPUProcess(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TerminateGPUProcess)(self, )
    }


    /// ```text
    /// /**
    ///     * Returns the GPU process pid, or -1 if there is no GPU process.
    ///     */
    /// ```
    ///

    /// `readonly attribute int32_t gpuProcessPid;`
    #[inline]
    pub unsafe fn GetGpuProcessPid(&self, aGpuProcessPid: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetGpuProcessPid)(self, aGpuProcessPid)
    }


    /// ```text
    /// /**
    ///    * Adds an EventStates bit to the element.
    ///    *
    ///    * The state string must be one of the following:
    ///    *   * (none yet; but for example "higlighted" for NS_EVENT_STATE_HIGHLIGHTED)
    ///    *
    ///    * The supported state strings are defined in kManuallyManagedStates
    ///    * in nsDOMWindowUtils.cpp.
    ///    */
    /// ```
    ///

    /// `void addManuallyManagedState (in Element element, in AString state);`
    #[inline]
    pub unsafe fn AddManuallyManagedState(&self, element: *const libc::c_void, state: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddManuallyManagedState)(self, element, state)
    }


    /// ```text
    /// /**
    ///    * Removes the specified EventStates bits from the element.
    ///    *
    ///    * See above for the strings that can be passed for |state|.
    ///    */
    /// ```
    ///

    /// `void removeManuallyManagedState (in Element element, in AString state);`
    #[inline]
    pub unsafe fn RemoveManuallyManagedState(&self, element: *const libc::c_void, state: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveManuallyManagedState)(self, element, state)
    }


    /// ```text
    /// /**
    ///    * Returns usage data for a given storage object.
    ///    *
    ///    * @param aStorage
    ///    *    The storage object to get usage data for.
    ///    */
    /// ```
    ///

    /// `int64_t getStorageUsage (in Storage aStorage);`
    #[inline]
    pub unsafe fn GetStorageUsage(&self, aStorage: *const libc::c_void, _retval: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetStorageUsage)(self, aStorage, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the directionality of a string using the first-strong character
    ///    * algorithm defined in http://unicode.org/reports/tr9/#P2.
    ///    *
    ///    * @param aString the string to retrieve the direction for.
    ///    * @return one of DIRECTION_LTR, DIRECTION_RTL or DIRECTION_NOT_SET depending
    ///    *         on the first-strong character found in the string.
    ///    */
    /// ```
    ///

    /// `long getDirectionFromText (in AString aString);`
    #[inline]
    pub unsafe fn GetDirectionFromText(&self, aString: *const ::nsstring::nsAString, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDirectionFromText)(self, aString, _retval)
    }


    /// ```text
    /// /**
    ///    * Calls FrameNeedsReflow on that root frame so that a layout flush
    ///    * will be necessary.
    ///    *
    ///    * This should only be used for testing.
    ///    */
    /// ```
    ///

    /// `void ensureDirtyRootFrame ();`
    #[inline]
    pub unsafe fn EnsureDirtyRootFrame(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnsureDirtyRootFrame)(self, )
    }


    /// ```text
    /// /**
    ///    * Capture the contents of the current WebRender frame and
    ///    * save them to a folder relative to the current working directory.
    ///    */
    /// ```
    ///

    /// `void wrCapture ();`
    #[inline]
    pub unsafe fn WrCapture(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).WrCapture)(self, )
    }


    /// ```text
    /// /**
    ///    * Start capturing the contents of the current WebRender frame and
    ///    * save them to a folder relative to the current working directory,
    ///    * as well as any frames generated from this point onward. If called
    ///    * again, it will stop capturing.
    ///    */
    /// ```
    ///

    /// `void wrToggleCaptureSequence ();`
    #[inline]
    pub unsafe fn WrToggleCaptureSequence(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).WrToggleCaptureSequence)(self, )
    }


    /// ```text
    /// /**
    ///    * Toggle recording of composition on and off.
    ///    *
    ///    * This is equivalent to calling |startCompositionRecorder()| or
    ///    * |stopCompositionRecorder(true)|.
    ///    */
    /// ```
    ///

    /// `Promise setCompositionRecording (in boolean aValue);`
    const _SetCompositionRecording: () = ();

    /// ```text
    /// /**
    ///    * Start the composition recorder.
    ///    *
    ///    * @return A promise that is resolved to true if the composion recorder was
    ///    *         started successfully.
    ///    */
    /// ```
    ///

    /// `Promise startCompositionRecording ();`
    const _StartCompositionRecording: () = ();

    /// ```text
    /// /**
    ///    * Stop the composition recorder.
    ///    *
    ///    * @param aWriteToDisk Whether or not the frames should be written to disk.
    ///    *                     If false, they will be returned in the promise.
    ///    * @return A promise that resolves when the frames have been collected.
    ///    *         When |aWriteToDisk| is true, the promise will resolve to |undefined|.
    ///    *         Otherwise, the promise will resolve to a |DOMCollectedFrames| dictionary,
    ///    *         which contains the timestamps and contents of the captured frames.
    ///    */
    /// ```
    ///

    /// `Promise stopCompositionRecording (in boolean aWriteToDisk);`
    const _StopCompositionRecording: () = ();

    /// ```text
    /// /**
    ///    * Returns whether the document we're associated to has recorded a given CSS
    ///    * property via the use counter mechanism.
    ///    *
    ///    * Throws if there's no document or the property is invalid.
    ///    */
    /// ```
    ///

    /// `bool isCssPropertyRecordedInUseCounter (in ACString aProperty);`
    #[inline]
    pub unsafe fn IsCssPropertyRecordedInUseCounter(&self, aProperty: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCssPropertyRecordedInUseCounter)(self, aProperty, _retval)
    }


    /// ```text
    /// /**
    ///    * Calls SetInitialViewport on the MobileViewportManager, which effectively
    ///    * causes it to refresh all of its internal state and update things that
    ///    * need updating.
    ///    */
    /// ```
    ///

    /// `void resetMobileViewportManager ();`
    #[inline]
    pub unsafe fn ResetMobileViewportManager(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetMobileViewportManager)(self, )
    }


    /// ```text
    /// /**
    ///    * NOTE: Currently works only on GTK+.
    ///    */
    /// ```
    ///

    /// `attribute ACString systemFont;`
    #[inline]
    pub unsafe fn GetSystemFont(&self, aSystemFont: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSystemFont)(self, aSystemFont)
    }


    /// ```text
    /// /**
    ///    * NOTE: Currently works only on GTK+.
    ///    */
    /// ```
    ///

    /// `attribute ACString systemFont;`
    #[inline]
    pub unsafe fn SetSystemFont(&self, aSystemFont: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetSystemFont)(self, aSystemFont)
    }


    /// ```text
    /// /**
    ///    * Returns the number of times this document for this window has
    ///    * been painted to the screen.
    ///    *
    ///    * Use this instead of window.mozPaintCount
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long long paintCount;`
    #[inline]
    pub unsafe fn GetPaintCount(&self, aPaintCount: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetPaintCount)(self, aPaintCount)
    }



    /// `void syncFlushCompositor ();`
    #[inline]
    pub unsafe fn SyncFlushCompositor(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SyncFlushCompositor)(self, )
    }



    /// `unsigned long long getLayersId ();`
    #[inline]
    pub unsafe fn GetLayersId(&self, _retval: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetLayersId)(self, _retval)
    }



    /// `readonly attribute bool usesOverlayScrollbars;`
    #[inline]
    pub unsafe fn GetUsesOverlayScrollbars(&self, aUsesOverlayScrollbars: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUsesOverlayScrollbars)(self, aUsesOverlayScrollbars)
    }



    /// `readonly attribute bool effectivelyThrottlesFrameRequests;`
    #[inline]
    pub unsafe fn GetEffectivelyThrottlesFrameRequests(&self, aEffectivelyThrottlesFrameRequests: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEffectivelyThrottlesFrameRequests)(self, aEffectivelyThrottlesFrameRequests)
    }



    /// `readonly attribute AString webrtcRawDeviceId;`
    #[inline]
    pub unsafe fn GetWebrtcRawDeviceId(&self, aWebrtcRawDeviceId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetWebrtcRawDeviceId)(self, aWebrtcRawDeviceId)
    }


}


/// `interface nsITranslationNodeList : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITranslationNodeList {
    vtable: *const nsITranslationNodeListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITranslationNodeList.
unsafe impl XpCom for nsITranslationNodeList {
    const IID: nsIID = nsID(0xc694e359, 0x7227, 0x4392,
        [0xa1, 0x38, 0x33, 0xc0, 0xcc, 0x1f, 0x15, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITranslationNodeList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITranslationNodeList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITranslationNodeListCoerce {
    /// Cheaply cast a value of this type from a `nsITranslationNodeList`.
    fn coerce_from(v: &nsITranslationNodeList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITranslationNodeListCoerce for nsITranslationNodeList {
    #[inline]
    fn coerce_from(v: &nsITranslationNodeList) -> &Self {
        v
    }
}

impl nsITranslationNodeList {
    /// Cast this `nsITranslationNodeList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITranslationNodeListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITranslationNodeList {
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
impl<T: nsISupportsCoerce> nsITranslationNodeListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITranslationNodeList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITranslationNodeList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITranslationNodeListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsITranslationNodeList, aLength: *mut u32) -> ::nserror::nsresult,

    /* Node item (in unsigned long index); */
    pub Item: unsafe extern "system" fn (this: *const nsITranslationNodeList, index: u32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* boolean isTranslationRootAtIndex (in unsigned long index); */
    pub IsTranslationRootAtIndex: unsafe extern "system" fn (this: *const nsITranslationNodeList, index: u32, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITranslationNodeList {


    /// `readonly attribute unsigned long length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }



    /// `Node item (in unsigned long index);`
    #[inline]
    pub unsafe fn Item(&self, index: u32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).Item)(self, index, _retval)
    }



    /// `boolean isTranslationRootAtIndex (in unsigned long index);`
    #[inline]
    pub unsafe fn IsTranslationRootAtIndex(&self, index: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsTranslationRootAtIndex)(self, index, _retval)
    }


}


/// `interface nsIJSRAIIHelper : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJSRAIIHelper {
    vtable: *const nsIJSRAIIHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJSRAIIHelper.
unsafe impl XpCom for nsIJSRAIIHelper {
    const IID: nsIID = nsID(0x52e5a996, 0xd0a9, 0x4efc,
        [0xa6, 0xfa, 0x24, 0x48, 0x9c, 0x53, 0x2b, 0x19]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJSRAIIHelper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJSRAIIHelper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJSRAIIHelperCoerce {
    /// Cheaply cast a value of this type from a `nsIJSRAIIHelper`.
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJSRAIIHelperCoerce for nsIJSRAIIHelper {
    #[inline]
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self {
        v
    }
}

impl nsIJSRAIIHelper {
    /// Cast this `nsIJSRAIIHelper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJSRAIIHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJSRAIIHelper {
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
impl<T: nsISupportsCoerce> nsIJSRAIIHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSRAIIHelper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJSRAIIHelper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJSRAIIHelperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void destruct (); */
    pub Destruct: unsafe extern "system" fn (this: *const nsIJSRAIIHelper) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJSRAIIHelper {

    /// ```text
    /// /**
    ///  * JS doesn't do RAII very well. We can use this interface to make remembering
    ///  * to destruct an object in a finally clause easier.
    ///  */
    /// ```
    ///

    /// `void destruct ();`
    #[inline]
    pub unsafe fn Destruct(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Destruct)(self, )
    }


}


