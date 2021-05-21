/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMWindowUtils.idl
 */

#ifndef __gen_nsIDOMWindowUtils_h__
#define __gen_nsIDOMWindowUtils_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsColor.h"
class gfxContext;
struct nsRect;
class nsIArray; /* forward declaration */

class nsICycleCollectorListener; /* forward declaration */

class nsIPreloadedStyleSheet; /* forward declaration */

class nsITransferable; /* forward declaration */

class nsIQueryContentEventResult; /* forward declaration */

class nsIDOMWindow; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIRunnable; /* forward declaration */

class nsITranslationNodeList; /* forward declaration */

class nsIJSRAIIHelper; /* forward declaration */

class nsIContentPermissionRequest; /* forward declaration */

class nsIObserver; /* forward declaration */

namespace mozilla {
namespace dom {
class Animation; /* webidl Animation */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class DOMRect; /* webidl DOMRect */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla

class nsINode; /* webidl Node */

class nsINodeList; /* webidl NodeList */

namespace mozilla {
namespace dom {
class Storage; /* webidl Storage */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMWindowUtils */
#define NS_IDOMWINDOWUTILS_IID_STR "4d6732ca-9da7-4176-b8a1-8dde15cd0bf9"

#define NS_IDOMWINDOWUTILS_IID \
  {0x4d6732ca, 0x9da7, 0x4176, \
    { 0xb8, 0xa1, 0x8d, 0xde, 0x15, 0xcd, 0x0b, 0xf9 }}

class NS_NO_VTABLE nsIDOMWindowUtils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMWINDOWUTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMWindowUtils;

  /* attribute unsigned short imageAnimationMode; */
  NS_IMETHOD GetImageAnimationMode(uint16_t *aImageAnimationMode) = 0;
  NS_IMETHOD SetImageAnimationMode(uint16_t aImageAnimationMode) = 0;

  /* readonly attribute boolean docCharsetIsForced; */
  NS_IMETHOD GetDocCharsetIsForced(bool *aDocCharsetIsForced) = 0;

  /* readonly attribute float physicalMillimeterInCSSPixels; */
  NS_IMETHOD GetPhysicalMillimeterInCSSPixels(float *aPhysicalMillimeterInCSSPixels) = 0;

  /* AString getDocumentMetadata (in AString aName); */
  NS_IMETHOD GetDocumentMetadata(const nsAString& aName, nsAString& _retval) = 0;

  /* [can_run_script] void updateLayerTree (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateLayerTree(void) = 0;

  /* readonly attribute unsigned long long lastTransactionId; */
  NS_IMETHOD GetLastTransactionId(uint64_t *aLastTransactionId) = 0;

  /* void getViewportInfo (in uint32_t aDisplayWidth, in uint32_t aDisplayHeight, out double aDefaultZoom, out boolean aAllowZoom, out double aMinZoom, out double aMaxZoom, out uint32_t aWidth, out uint32_t aHeight, out boolean aAutoSize); */
  NS_IMETHOD GetViewportInfo(uint32_t aDisplayWidth, uint32_t aDisplayHeight, double *aDefaultZoom, bool *aAllowZoom, double *aMinZoom, double *aMaxZoom, uint32_t *aWidth, uint32_t *aHeight, bool *aAutoSize) = 0;

  /* AString getViewportFitInfo (); */
  NS_IMETHOD GetViewportFitInfo(nsAString& _retval) = 0;

  /* void getContentViewerSize (out uint32_t aDisplayWidth, out uint32_t aDisplayHeight); */
  NS_IMETHOD GetContentViewerSize(uint32_t *aDisplayWidth, uint32_t *aDisplayHeight) = 0;

  /* void setDisplayPortForElement (in float aXPx, in float aYPx, in float aWidthPx, in float aHeightPx, in Element aElement, in uint32_t aPriority); */
  NS_IMETHOD SetDisplayPortForElement(float aXPx, float aYPx, float aWidthPx, float aHeightPx, mozilla::dom::Element *aElement, uint32_t aPriority) = 0;

  /* void setDisplayPortMarginsForElement (in float aLeftMargin, in float aTopMargin, in float aRightMargin, in float aBottomMargin, in Element aElement, in uint32_t aPriority); */
  NS_IMETHOD SetDisplayPortMarginsForElement(float aLeftMargin, float aTopMargin, float aRightMargin, float aBottomMargin, mozilla::dom::Element *aElement, uint32_t aPriority) = 0;

  /* void setDisplayPortBaseForElement (in int32_t aX, in int32_t aY, in int32_t aWidth, in int32_t aHeight, in Element aElement); */
  NS_IMETHOD SetDisplayPortBaseForElement(int32_t aX, int32_t aY, int32_t aWidth, int32_t aHeight, mozilla::dom::Element *aElement) = 0;

  /* void getScrollbarSizes (in Element aElement, out uint32_t aVerticalScrollbarWidth, out uint32_t aHorizontalScrollbarHeight); */
  NS_IMETHOD GetScrollbarSizes(mozilla::dom::Element *aElement, uint32_t *aVerticalScrollbarWidth, uint32_t *aHorizontalScrollbarHeight) = 0;

  /* void setResolutionAndScaleTo (in float aResolution); */
  NS_IMETHOD SetResolutionAndScaleTo(float aResolution) = 0;

  /* float getResolution (); */
  NS_IMETHOD GetResolution(float *_retval) = 0;

  /* void setRestoreResolution (in float aResolution, in uint32_t aDisplayWidth, in uint32_t aDisplayHeight); */
  NS_IMETHOD SetRestoreResolution(float aResolution, uint32_t aDisplayWidth, uint32_t aDisplayHeight) = 0;

  /* attribute boolean isFirstPaint; */
  NS_IMETHOD GetIsFirstPaint(bool *aIsFirstPaint) = 0;
  NS_IMETHOD SetIsFirstPaint(bool aIsFirstPaint) = 0;

  /* uint32_t getPresShellId (); */
  NS_IMETHOD GetPresShellId(uint32_t *_retval) = 0;

  /* boolean isCORSSafelistedRequestHeader (in ACString name, in ACString value); */
  NS_IMETHOD IsCORSSafelistedRequestHeader(const nsACString& name, const nsACString& value, bool *_retval) = 0;

  enum {
    MODIFIER_ALT = 1,
    MODIFIER_CONTROL = 2,
    MODIFIER_SHIFT = 4,
    MODIFIER_META = 8,
    MODIFIER_ALTGRAPH = 16,
    MODIFIER_CAPSLOCK = 32,
    MODIFIER_FN = 64,
    MODIFIER_FNLOCK = 128,
    MODIFIER_NUMLOCK = 256,
    MODIFIER_SCROLLLOCK = 512,
    MODIFIER_SYMBOL = 1024,
    MODIFIER_SYMBOLLOCK = 2048,
    MODIFIER_OS = 4096
  };

  /* [can_run_script,optional_argc] boolean sendMouseEvent (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEvent(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc, bool *_retval) = 0;

  /* [can_run_script] boolean sendTouchEvent (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEvent(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) = 0;

  /* [can_run_script,optional_argc] void sendMouseEventToWindow (in AString aType, in float aX, in float aY, in long aButton, in long aClickCount, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame, [optional] in float aPressure, [optional] in unsigned short aInputSourceArg, [optional] in boolean aIsDOMEventSynthesized, [optional] in boolean aIsWidgetEventSynthesized, [optional] in long aButtons, [optional] in unsigned long aIdentifier); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEventToWindow(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc) = 0;

  /* [can_run_script] boolean sendTouchEventToWindow (in AString aType, in Array<uint32_t> aIdentifiers, in Array<int32_t> aXs, in Array<int32_t> aYs, in Array<uint32_t> aRxs, in Array<uint32_t> aRys, in Array<float> aRotationAngles, in Array<float> aForces, in long aModifiers, [optional] in boolean aIgnoreRootScrollFrame); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEventToWindow(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) = 0;

  enum {
    WHEEL_EVENT_CAUSED_BY_NO_LINE_OR_PAGE_DELTA_DEVICE = 1U,
    WHEEL_EVENT_CAUSED_BY_MOMENTUM = 2U,
    WHEEL_EVENT_CUSTOMIZED_BY_USER_PREFS = 4U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_ZERO = 16U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_POSITIVE = 32U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_X_NEGATIVE = 64U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_ZERO = 256U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_POSITIVE = 512U,
    WHEEL_EVENT_EXPECTED_OVERFLOW_DELTA_Y_NEGATIVE = 1024U
  };

  /* void sendWheelEvent (in float aX, in float aY, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aDeltaMode, in long aModifiers, in long aLineOrPageDeltaX, in long aLineOrPageDeltaY, in unsigned long aOptions); */
  NS_IMETHOD SendWheelEvent(float aX, float aY, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aDeltaMode, int32_t aModifiers, int32_t aLineOrPageDeltaX, int32_t aLineOrPageDeltaY, uint32_t aOptions) = 0;

  /* void sendNativeKeyEvent (in long aNativeKeyboardLayout, in long aNativeKeyCode, in long aModifierFlags, in AString aCharacters, in AString aUnmodifiedCharacters, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeKeyEvent(int32_t aNativeKeyboardLayout, int32_t aNativeKeyCode, int32_t aModifierFlags, const nsAString& aCharacters, const nsAString& aUnmodifiedCharacters, nsIObserver *aObserver) = 0;

  /* void sendNativeMouseEvent (in long aScreenX, in long aScreenY, in long aNativeMessage, in long aModifierFlags, in Element aElement, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeMouseEvent(int32_t aScreenX, int32_t aScreenY, int32_t aNativeMessage, int32_t aModifierFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) = 0;

  /* void sendNativeMouseMove (in long aScreenX, in long aScreenY, in Element aElement, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeMouseMove(int32_t aScreenX, int32_t aScreenY, mozilla::dom::Element *aElement, nsIObserver *aObserver) = 0;

  /* void suppressAnimation (in boolean aSuppress); */
  NS_IMETHOD SuppressAnimation(bool aSuppress) = 0;

  enum {
    MOUSESCROLL_PREFER_WIDGET_AT_POINT = 1U,
    MOUSESCROLL_SCROLL_LINES = 2U,
    MOUSESCROLL_WIN_SCROLL_LPARAM_NOT_NULL = 65536U
  };

  /* void sendNativeMouseScrollEvent (in long aScreenX, in long aScreenY, in unsigned long aNativeMessage, in double aDeltaX, in double aDeltaY, in double aDeltaZ, in unsigned long aModifierFlags, in unsigned long aAdditionalFlags, in Element aElement, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeMouseScrollEvent(int32_t aScreenX, int32_t aScreenY, uint32_t aNativeMessage, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aModifierFlags, uint32_t aAdditionalFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) = 0;

  enum {
    TOUCH_HOVER = 1,
    TOUCH_CONTACT = 2,
    TOUCH_REMOVE = 4,
    TOUCH_CANCEL = 8,
    PHASE_BEGIN = 0,
    PHASE_UPDATE = 1,
    PHASE_END = 2
  };

  /* void sendNativeTouchPoint (in unsigned long aPointerId, in unsigned long aTouchState, in long aScreenX, in long aScreenY, in double aPressure, in unsigned long aOrientation, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeTouchPoint(uint32_t aPointerId, uint32_t aTouchState, int32_t aScreenX, int32_t aScreenY, double aPressure, uint32_t aOrientation, nsIObserver *aObserver) = 0;

  /* void sendNativeTouchpadPinch (in unsigned long aEventPhase, in float aScale, in long aScreenX, in long aScreenY, in long aModifierFlags); */
  NS_IMETHOD SendNativeTouchpadPinch(uint32_t aEventPhase, float aScale, int32_t aScreenX, int32_t aScreenY, int32_t aModifierFlags) = 0;

  /* void sendNativeTouchTap (in long aScreenX, in long aScreenY, in boolean aLongTap, [optional] in nsIObserver aObserver); */
  NS_IMETHOD SendNativeTouchTap(int32_t aScreenX, int32_t aScreenY, bool aLongTap, nsIObserver *aObserver) = 0;

  /* void clearNativeTouchSequence ([optional] in nsIObserver aObserver); */
  NS_IMETHOD ClearNativeTouchSequence(nsIObserver *aObserver) = 0;

  /* void clearSharedStyleSheetCache (); */
  NS_IMETHOD ClearSharedStyleSheetCache(void) = 0;

  /* readonly attribute unsigned long parsedStyleSheets; */
  NS_IMETHOD GetParsedStyleSheets(uint32_t *aParsedStyleSheets) = 0;

  /* void activateNativeMenuItemAt (in AString indexString); */
  NS_IMETHOD ActivateNativeMenuItemAt(const nsAString& indexString) = 0;

  /* void forceUpdateNativeMenuAt (in AString indexString); */
  NS_IMETHOD ForceUpdateNativeMenuAt(const nsAString& indexString) = 0;

  /* AString GetSelectionAsPlaintext (); */
  NS_IMETHOD GetSelectionAsPlaintext(nsAString& _retval) = 0;

  /* void garbageCollect ([optional] in nsICycleCollectorListener aListener); */
  NS_IMETHOD GarbageCollect(nsICycleCollectorListener *aListener) = 0;

  /* void cycleCollect ([optional] in nsICycleCollectorListener aListener); */
  NS_IMETHOD CycleCollect(nsICycleCollectorListener *aListener) = 0;

  /* void runNextCollectorTimer (); */
  NS_IMETHOD RunNextCollectorTimer(void) = 0;

  /* void sendSimpleGestureEvent (in AString aType, in float aX, in float aY, in unsigned long aDirection, in double aDelta, in long aModifiers, [optional] in unsigned long aClickCount); */
  NS_IMETHOD SendSimpleGestureEvent(const nsAString& aType, float aX, float aY, uint32_t aDirection, double aDelta, int32_t aModifiers, uint32_t aClickCount) = 0;

  /* Element elementFromPoint (in float aX, in float aY, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout); */
  NS_IMETHOD ElementFromPoint(float aX, float aY, bool aIgnoreRootScrollFrame, bool aFlushLayout, mozilla::dom::Element **_retval) = 0;

  /* NodeList nodesFromRect (in float aX, in float aY, in float aTopSize, in float aRightSize, in float aBottomSize, in float aLeftSize, in boolean aIgnoreRootScrollFrame, in boolean aFlushLayout, in boolean aOnlyVisible, [optional] in float aTransparencyThreshold); */
  NS_IMETHOD NodesFromRect(float aX, float aY, float aTopSize, float aRightSize, float aBottomSize, float aLeftSize, bool aIgnoreRootScrollFrame, bool aFlushLayout, bool aOnlyVisible, float aTransparencyThreshold, nsINodeList **_retval) = 0;

  /* nsITranslationNodeList getTranslationNodes (in Node aRoot); */
  NS_IMETHOD GetTranslationNodes(nsINode *aRoot, nsITranslationNodeList **_retval) = 0;

  /* uint32_t compareCanvases (in nsISupports aCanvas1, in nsISupports aCanvas2, out unsigned long aMaxDifference); */
  NS_IMETHOD CompareCanvases(nsISupports *aCanvas1, nsISupports *aCanvas2, uint32_t *aMaxDifference, uint32_t *_retval) = 0;

  /* readonly attribute boolean isMozAfterPaintPending; */
  NS_IMETHOD GetIsMozAfterPaintPending(bool *aIsMozAfterPaintPending) = 0;

  /* readonly attribute boolean isInputTaskManagerSuspended; */
  NS_IMETHOD GetIsInputTaskManagerSuspended(bool *aIsInputTaskManagerSuspended) = 0;

  /* void suppressEventHandling (in boolean aSuppress); */
  NS_IMETHOD SuppressEventHandling(bool aSuppress) = 0;

  /* void disableNonTestMouseEvents (in boolean aDisable); */
  NS_IMETHOD DisableNonTestMouseEvents(bool aDisable) = 0;

  /* void getScrollXY (in boolean aFlushLayout, out long aScrollX, out long aScrollY); */
  NS_IMETHOD GetScrollXY(bool aFlushLayout, int32_t *aScrollX, int32_t *aScrollY) = 0;

  /* void getScrollXYFloat (in boolean aFlushLayout, out float aScrollX, out float aScrollY); */
  NS_IMETHOD GetScrollXYFloat(bool aFlushLayout, float *aScrollX, float *aScrollY) = 0;

  /* void getScrollbarSize (in boolean aFlushLayout, out long aWidth, out long aHeight); */
  NS_IMETHOD GetScrollbarSize(bool aFlushLayout, int32_t *aWidth, int32_t *aHeight) = 0;

  /* DOMRect getBoundsWithoutFlushing (in Element aElement); */
  NS_IMETHOD GetBoundsWithoutFlushing(mozilla::dom::Element *aElement, mozilla::dom::DOMRect **_retval) = 0;

  enum {
    UPDATE_TYPE_RESTORE = 0,
    UPDATE_TYPE_MAIN_THREAD = 1,
    SCROLL_MODE_INSTANT = 0,
    SCROLL_MODE_SMOOTH = 1
  };

  /* void scrollToVisual (in float aOffsetX, in float aOffsetY, in long aUpdateType, in long aScrollMode); */
  NS_IMETHOD ScrollToVisual(float aOffsetX, float aOffsetY, int32_t aUpdateType, int32_t aScrollMode) = 0;

  /* void getVisualViewportOffsetRelativeToLayoutViewport (out float aOffsetX, out float aOffsetY); */
  NS_IMETHOD GetVisualViewportOffsetRelativeToLayoutViewport(float *aOffsetX, float *aOffsetY) = 0;

  /* void getVisualViewportOffset (out long aOffsetX, out long aOffsetY); */
  NS_IMETHOD GetVisualViewportOffset(int32_t *aOffsetX, int32_t *aOffsetY) = 0;

  /* DOMRect transformRectLayoutToVisual (in float aX, in float aY, in float aWidth, in float aHeight); */
  NS_IMETHOD TransformRectLayoutToVisual(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) = 0;

  /* DOMRect toScreenRect (in float aX, in float aY, in float aWidth, in float aHeight); */
  NS_IMETHOD ToScreenRect(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) = 0;

  /* [can_run_script] void setDynamicToolbarMaxHeight (in uint32_t aHeightInScreen); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDynamicToolbarMaxHeight(uint32_t aHeightInScreen) = 0;

  enum {
    FLUSH_NONE = -1,
    FLUSH_STYLE = 0,
    FLUSH_LAYOUT = 1,
    FLUSH_DISPLAY = 2
  };

  /* bool needsFlush (in long aFlushtype); */
  NS_IMETHOD NeedsFlush(int32_t aFlushtype, bool *_retval) = 0;

  /* void flushLayoutWithoutThrottledAnimations (); */
  NS_IMETHOD FlushLayoutWithoutThrottledAnimations(void) = 0;

  /* DOMRect getRootBounds (); */
  NS_IMETHOD GetRootBounds(mozilla::dom::DOMRect **_retval) = 0;

  /* readonly attribute boolean IMEIsOpen; */
  NS_IMETHOD GetIMEIsOpen(bool *aIMEIsOpen) = 0;

  enum {
    IME_STATUS_DISABLED = 0U,
    IME_STATUS_ENABLED = 1U,
    IME_STATUS_PASSWORD = 2U
  };

  /* readonly attribute unsigned long IMEStatus; */
  NS_IMETHOD GetIMEStatus(uint32_t *aIMEStatus) = 0;

  /* readonly attribute float screenPixelsPerCSSPixel; */
  NS_IMETHOD GetScreenPixelsPerCSSPixel(float *aScreenPixelsPerCSSPixel) = 0;

  /* readonly attribute float screenPixelsPerCSSPixelNoOverride; */
  NS_IMETHOD GetScreenPixelsPerCSSPixelNoOverride(float *aScreenPixelsPerCSSPixelNoOverride) = 0;

  /* readonly attribute float fullZoom; */
  NS_IMETHOD GetFullZoom(float *aFullZoom) = 0;

  /* [can_run_script] boolean dispatchDOMEventViaPresShellForTesting (in Node aTarget, in Event aEvent); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DispatchDOMEventViaPresShellForTesting(nsINode *aTarget, mozilla::dom::Event *aEvent, bool *_retval) = 0;

  /* boolean dispatchEventToChromeOnly (in EventTarget aTarget, in Event aEvent); */
  NS_IMETHOD DispatchEventToChromeOnly(mozilla::dom::EventTarget *aTarget, mozilla::dom::Event *aEvent, bool *_retval) = 0;

  /* [implicit_jscontext] string getClassName (in jsval aObject); */
  NS_IMETHOD GetClassName(JS::HandleValue aObject, JSContext* cx, char * *_retval) = 0;

  /* void sendContentCommandEvent (in AString aType, [optional] in nsITransferable aTransferable); */
  NS_IMETHOD SendContentCommandEvent(const nsAString& aType, nsITransferable *aTransferable) = 0;

  enum {
    QUERY_CONTENT_FLAG_USE_NATIVE_LINE_BREAK = 0U,
    QUERY_CONTENT_FLAG_USE_XP_LINE_BREAK = 1U,
    QUERY_CONTENT_FLAG_SELECTION_SPELLCHECK = 2U,
    QUERY_CONTENT_FLAG_SELECTION_IME_RAWINPUT = 4U,
    QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDRAWTEXT = 8U,
    QUERY_CONTENT_FLAG_SELECTION_IME_CONVERTEDTEXT = 16U,
    QUERY_CONTENT_FLAG_SELECTION_IME_SELECTEDCONVERTEDTEXT = 32U,
    QUERY_CONTENT_FLAG_SELECTION_ACCESSIBILITY = 64U,
    QUERY_CONTENT_FLAG_SELECTION_FIND = 128U,
    QUERY_CONTENT_FLAG_SELECTION_URLSECONDARY = 256U,
    QUERY_CONTENT_FLAG_SELECTION_URLSTRIKEOUT = 512U,
    QUERY_CONTENT_FLAG_OFFSET_RELATIVE_TO_INSERTION_POINT = 1024U
  };

  /* nsIQueryContentEventResult sendQueryContentEvent (in unsigned long aType, in long long aOffset, in unsigned long aLength, in long aX, in long aY, [optional] in unsigned long aAdditionalFlags); */
  NS_IMETHOD SendQueryContentEvent(uint32_t aType, int64_t aOffset, uint32_t aLength, int32_t aX, int32_t aY, uint32_t aAdditionalFlags, nsIQueryContentEventResult **_retval) = 0;

  enum {
    QUERY_SELECTED_TEXT = 3200U,
    QUERY_TEXT_CONTENT = 3201U,
    QUERY_CARET_RECT = 3203U,
    QUERY_TEXT_RECT = 3204U,
    QUERY_EDITOR_RECT = 3205U,
    QUERY_CHARACTER_AT_POINT = 3208U,
    QUERY_TEXT_RECT_ARRAY = 3209U
  };

  /* void remoteFrameFullscreenChanged (in Element aFrameElement); */
  NS_IMETHOD RemoteFrameFullscreenChanged(mozilla::dom::Element *aFrameElement) = 0;

  /* void remoteFrameFullscreenReverted (); */
  NS_IMETHOD RemoteFrameFullscreenReverted(void) = 0;

  /* boolean handleFullscreenRequests (); */
  NS_IMETHOD HandleFullscreenRequests(bool *_retval) = 0;

  /* void exitFullscreen (); */
  NS_IMETHOD ExitFullscreen(void) = 0;

  enum {
    SELECTION_SET_FLAG_USE_NATIVE_LINE_BREAK = 0U,
    SELECTION_SET_FLAG_USE_XP_LINE_BREAK = 1U,
    SELECTION_SET_FLAG_REVERSE = 2U
  };

  /* boolean sendSelectionSetEvent (in unsigned long aOffset, in unsigned long aLength, [optional] in unsigned long aAdditionalFlags); */
  NS_IMETHOD SendSelectionSetEvent(uint32_t aOffset, uint32_t aLength, uint32_t aAdditionalFlags, bool *_retval) = 0;

  enum {
    SELECT_CHARACTER = 0U,
    SELECT_CLUSTER = 1U,
    SELECT_WORD = 2U,
    SELECT_LINE = 3U,
    SELECT_BEGINLINE = 4U,
    SELECT_ENDLINE = 5U,
    SELECT_PARAGRAPH = 6U,
    SELECT_WORDNOSPACE = 7U
  };

  /* [can_run_script] boolean selectAtPoint (in float aX, in float aY, in unsigned long aSelectBehavior); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAtPoint(float aX, float aY, uint32_t aSelectBehavior, bool *_retval) = 0;

  /* AString getVisitedDependentComputedStyle (in Element aElement, in AString aPseudoElement, in AString aPropertyName); */
  NS_IMETHOD GetVisitedDependentComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aPropertyName, nsAString& _retval) = 0;

  /* readonly attribute unsigned long long deprecatedOuterWindowID; */
  NS_IMETHOD GetDeprecatedOuterWindowID(uint64_t *aDeprecatedOuterWindowID) = 0;

  /* void enterModalState (); */
  NS_IMETHOD EnterModalState(void) = 0;

  /* void leaveModalState (); */
  NS_IMETHOD LeaveModalState(void) = 0;

  /* boolean isInModalState (); */
  NS_IMETHOD IsInModalState(bool *_retval) = 0;

  /* attribute boolean desktopModeViewport; */
  NS_IMETHOD GetDesktopModeViewport(bool *aDesktopModeViewport) = 0;
  NS_IMETHOD SetDesktopModeViewport(bool aDesktopModeViewport) = 0;

  /* void suspendTimeouts (); */
  NS_IMETHOD SuspendTimeouts(void) = 0;

  /* void resumeTimeouts (); */
  NS_IMETHOD ResumeTimeouts(void) = 0;

  /* readonly attribute AString layerManagerType; */
  NS_IMETHOD GetLayerManagerType(nsAString& aLayerManagerType) = 0;

  /* readonly attribute boolean layerManagerRemote; */
  NS_IMETHOD GetLayerManagerRemote(bool *aLayerManagerRemote) = 0;

  /* readonly attribute boolean usingAdvancedLayers; */
  NS_IMETHOD GetUsingAdvancedLayers(bool *aUsingAdvancedLayers) = 0;

  /* readonly attribute boolean isWebRenderRequested; */
  NS_IMETHOD GetIsWebRenderRequested(bool *aIsWebRenderRequested) = 0;

  /* readonly attribute AString currentAudioBackend; */
  NS_IMETHOD GetCurrentAudioBackend(nsAString& aCurrentAudioBackend) = 0;

  /* readonly attribute unsigned long currentMaxAudioChannels; */
  NS_IMETHOD GetCurrentMaxAudioChannels(uint32_t *aCurrentMaxAudioChannels) = 0;

  /* Promise defaultDevicesRoundTripLatency (); */
  NS_IMETHOD DefaultDevicesRoundTripLatency(::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute unsigned long currentPreferredSampleRate; */
  NS_IMETHOD GetCurrentPreferredSampleRate(uint32_t *aCurrentPreferredSampleRate) = 0;

  enum {
    AUDIO_INPUT = 0U,
    AUDIO_OUTPUT = 1U
  };

  /* nsIArray audioDevices (in unsigned short aSide); */
  NS_IMETHOD AudioDevices(uint16_t aSide, nsIArray **_retval) = 0;

  /* void startFrameTimeRecording ([retval] out unsigned long startIndex); */
  NS_IMETHOD StartFrameTimeRecording(uint32_t *startIndex) = 0;

  /* Array<float> stopFrameTimeRecording (in unsigned long startIndex); */
  NS_IMETHOD StopFrameTimeRecording(uint32_t startIndex, nsTArray<float >& _retval) = 0;

  /* readonly attribute float displayDPI; */
  NS_IMETHOD GetDisplayDPI(float *aDisplayDPI) = 0;

  /* void advanceTimeAndRefresh (in long long aMilliseconds); */
  NS_IMETHOD AdvanceTimeAndRefresh(int64_t aMilliseconds) = 0;

  /* void restoreNormalRefresh (); */
  NS_IMETHOD RestoreNormalRefresh(void) = 0;

  /* readonly attribute bool isTestControllingRefreshes; */
  NS_IMETHOD GetIsTestControllingRefreshes(bool *aIsTestControllingRefreshes) = 0;

  /* readonly attribute bool asyncPanZoomEnabled; */
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) = 0;

  /* void setAsyncScrollOffset (in Element aElement, in float aX, in float aY); */
  NS_IMETHOD SetAsyncScrollOffset(mozilla::dom::Element *aElement, float aX, float aY) = 0;

  /* void setAsyncZoom (in Element aRootElement, in float aValue); */
  NS_IMETHOD SetAsyncZoom(mozilla::dom::Element *aRootElement, float aValue) = 0;

  /* bool flushApzRepaints (); */
  NS_IMETHOD FlushApzRepaints(bool *_retval) = 0;

  /* void disableApzForElement (in Element aElement); */
  NS_IMETHOD DisableApzForElement(mozilla::dom::Element *aElement) = 0;

  /* [can_run_script] void zoomToFocusedInput (); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ZoomToFocusedInput(void) = 0;

  /* double computeAnimationDistance (in Element element, in AString property, in AString value1, in AString value2); */
  NS_IMETHOD ComputeAnimationDistance(mozilla::dom::Element *element, const nsAString& property, const nsAString& value1, const nsAString& value2, double *_retval) = 0;

  /* AString getUnanimatedComputedStyle (in Element aElement, in AString aPseudoElement, in AString aProperty, in long aFlushType); */
  NS_IMETHOD GetUnanimatedComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aProperty, int32_t aFlushType, nsAString& _retval) = 0;

  /* readonly attribute AString focusedInputType; */
  NS_IMETHOD GetFocusedInputType(nsAString& aFocusedInputType) = 0;

  /* readonly attribute AString focusedActionHint; */
  NS_IMETHOD GetFocusedActionHint(nsAString& aFocusedActionHint) = 0;

  /* readonly attribute AString focusedInputMode; */
  NS_IMETHOD GetFocusedInputMode(nsAString& aFocusedInputMode) = 0;

  /* readonly attribute AString focusedAutocapitalize; */
  NS_IMETHOD GetFocusedAutocapitalize(nsAString& aFocusedAutocapitalize) = 0;

  /* nsViewID getViewId (in Element aElement); */
  NS_IMETHOD GetViewId(mozilla::dom::Element *aElement, nsViewID *_retval) = 0;

  /* boolean leafLayersPartitionWindow (); */
  NS_IMETHOD LeafLayersPartitionWindow(bool *_retval) = 0;

  /* boolean checkAndClearPaintedState (in Element aElement); */
  NS_IMETHOD CheckAndClearPaintedState(mozilla::dom::Element *aElement, bool *_retval) = 0;

  /* boolean checkAndClearDisplayListState (in Element aElement); */
  NS_IMETHOD CheckAndClearDisplayListState(mozilla::dom::Element *aElement, bool *_retval) = 0;

  /* boolean isPartOfOpaqueLayer (in Element aElement); */
  NS_IMETHOD IsPartOfOpaqueLayer(mozilla::dom::Element *aElement, bool *_retval) = 0;

  /* unsigned long numberOfAssignedPaintedLayers (in Array<Element> aElements); */
  NS_IMETHOD NumberOfAssignedPaintedLayers(const nsTArray<RefPtr<mozilla::dom::Element>>& aElements, uint32_t *_retval) = 0;

  /* [implicit_jscontext] long long getFileId (in jsval aFile); */
  NS_IMETHOD GetFileId(JS::HandleValue aFile, JSContext* cx, int64_t *_retval) = 0;

  /* [implicit_jscontext] AString getFilePath (in jsval aFile); */
  NS_IMETHOD GetFilePath(JS::HandleValue aFile, JSContext* cx, nsAString& _retval) = 0;

  /* [implicit_jscontext] boolean getFileReferences (in AString aDatabaseName, in long long aId, [optional] in jsval aOptions, [optional] out long aRefCnt, [optional] out long aDBRefCnt); */
  NS_IMETHOD GetFileReferences(const nsAString& aDatabaseName, int64_t aId, JS::HandleValue aOptions, int32_t *aRefCnt, int32_t *aDBRefCnt, JSContext* cx, bool *_retval) = 0;

  /* void flushPendingFileDeletions (); */
  NS_IMETHOD FlushPendingFileDeletions(void) = 0;

  /* [implicit_jscontext] void startPCCountProfiling (); */
  NS_IMETHOD StartPCCountProfiling(JSContext* cx) = 0;

  /* [implicit_jscontext] void stopPCCountProfiling (); */
  NS_IMETHOD StopPCCountProfiling(JSContext* cx) = 0;

  /* [implicit_jscontext] void purgePCCounts (); */
  NS_IMETHOD PurgePCCounts(JSContext* cx) = 0;

  /* [implicit_jscontext] long getPCCountScriptCount (); */
  NS_IMETHOD GetPCCountScriptCount(JSContext* cx, int32_t *_retval) = 0;

  /* [implicit_jscontext] AString getPCCountScriptSummary (in long script); */
  NS_IMETHOD GetPCCountScriptSummary(int32_t script, JSContext* cx, nsAString& _retval) = 0;

  /* [implicit_jscontext] AString getPCCountScriptContents (in long script); */
  NS_IMETHOD GetPCCountScriptContents(int32_t script, JSContext* cx, nsAString& _retval) = 0;

  /* readonly attribute boolean paintingSuppressed; */
  NS_IMETHOD GetPaintingSuppressed(bool *aPaintingSuppressed) = 0;

  /* [implicit_jscontext] readonly attribute jsval plugins; */
  NS_IMETHOD GetPlugins(JSContext* cx, JS::MutableHandleValue aPlugins) = 0;

  /* void setVisualViewportSize (in float aWidth, in float aHeight); */
  NS_IMETHOD SetVisualViewportSize(float aWidth, float aHeight) = 0;

  /* void disableDialogs (); */
  NS_IMETHOD DisableDialogs(void) = 0;

  /* void enableDialogs (); */
  NS_IMETHOD EnableDialogs(void) = 0;

  /* bool areDialogsEnabled (); */
  NS_IMETHOD AreDialogsEnabled(bool *_retval) = 0;

  enum {
    AGENT_SHEET = 0U,
    USER_SHEET = 1U,
    AUTHOR_SHEET = 2U
  };

  /* void loadSheet (in nsIURI sheetURI, in unsigned long type); */
  NS_IMETHOD LoadSheet(nsIURI *sheetURI, uint32_t type) = 0;

  /* void loadSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
  NS_IMETHOD LoadSheetUsingURIString(const nsACString& sheetURI, uint32_t type) = 0;

  /* void addSheet (in nsIPreloadedStyleSheet sheet, in unsigned long type); */
  NS_IMETHOD AddSheet(nsIPreloadedStyleSheet *sheet, uint32_t type) = 0;

  /* void removeSheet (in nsIURI sheetURI, in unsigned long type); */
  NS_IMETHOD RemoveSheet(nsIURI *sheetURI, uint32_t type) = 0;

  /* void removeSheetUsingURIString (in ACString sheetURI, in unsigned long type); */
  NS_IMETHOD RemoveSheetUsingURIString(const nsACString& sheetURI, uint32_t type) = 0;

  /* readonly attribute boolean isHandlingUserInput; */
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) = 0;

  /* readonly attribute double millisSinceLastUserInput; */
  NS_IMETHOD GetMillisSinceLastUserInput(double *aMillisSinceLastUserInput) = 0;

  /* void allowScriptsToClose (); */
  NS_IMETHOD AllowScriptsToClose(void) = 0;

  /* readonly attribute boolean isParentWindowMainWidgetVisible; */
  NS_IMETHOD GetIsParentWindowMainWidgetVisible(bool *aIsParentWindowMainWidgetVisible) = 0;

  /* boolean isNodeDisabledForEvents (in Node aNode); */
  NS_IMETHOD IsNodeDisabledForEvents(nsINode *aNode, bool *_retval) = 0;

  /* attribute boolean paintFlashing; */
  NS_IMETHOD GetPaintFlashing(bool *aPaintFlashing) = 0;
  NS_IMETHOD SetPaintFlashing(bool aPaintFlashing) = 0;

  /* AString getOMTAStyle (in Element aElement, in AString aProperty, [optional] in AString aPseudoElement); */
  NS_IMETHOD GetOMTAStyle(mozilla::dom::Element *aElement, const nsAString& aProperty, const nsAString& aPseudoElement, nsAString& _retval) = 0;

  /* AString getOMTCTransform (in Element aElement, [optional] in AString aPseudoElement); */
  NS_IMETHOD GetOMTCTransform(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, nsAString& _retval) = 0;

  /* bool isAnimationInPendingTracker (in Animation aAnimation); */
  NS_IMETHOD IsAnimationInPendingTracker(mozilla::dom::Animation *aAnimation, bool *_retval) = 0;

  /* nsIJSRAIIHelper setHandlingUserInput (in boolean aHandlingInput); */
  NS_IMETHOD SetHandlingUserInput(bool aHandlingInput, nsIJSRAIIHelper **_retval) = 0;

  /* [implicit_jscontext] jsval getContentAPZTestData (); */
  NS_IMETHOD GetContentAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval getCompositorAPZTestData (); */
  NS_IMETHOD GetCompositorAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void postRestyleSelfEvent (in Element aElement); */
  NS_IMETHOD PostRestyleSelfEvent(mozilla::dom::Element *aElement) = 0;

  /* void xpconnectArgument (in nsISupports aObj); */
  NS_IMETHOD XpconnectArgument(nsISupports *aObj) = 0;

  /* void askPermission (in nsIContentPermissionRequest aRequest); */
  NS_IMETHOD AskPermission(nsIContentPermissionRequest *aRequest) = 0;

  /* readonly attribute unsigned long long restyleGeneration; */
  NS_IMETHOD GetRestyleGeneration(uint64_t *aRestyleGeneration) = 0;

  /* readonly attribute unsigned long long framesConstructed; */
  NS_IMETHOD GetFramesConstructed(uint64_t *aFramesConstructed) = 0;

  /* readonly attribute unsigned long long framesReflowed; */
  NS_IMETHOD GetFramesReflowed(uint64_t *aFramesReflowed) = 0;

  /* void setChromeMargin (in int32_t aTop, in int32_t aRight, in int32_t aBottom, in int32_t aLeft); */
  NS_IMETHOD SetChromeMargin(int32_t aTop, int32_t aRight, int32_t aBottom, int32_t aLeft) = 0;

  /* [implicit_jscontext] jsval getFrameUniformityTestData (); */
  NS_IMETHOD GetFrameUniformityTestData(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void enterChaosMode (); */
  NS_IMETHOD EnterChaosMode(void) = 0;

  /* void leaveChaosMode (); */
  NS_IMETHOD LeaveChaosMode(void) = 0;

  /* void triggerDeviceReset (); */
  NS_IMETHOD TriggerDeviceReset(void) = 0;

  /* bool hasRuleProcessorUsedByMultipleStyleSets (in unsigned long aSheetType); */
  NS_IMETHOD HasRuleProcessorUsedByMultipleStyleSets(uint32_t aSheetType, bool *_retval) = 0;

  /* void respectDisplayPortSuppression (in boolean aEnabled); */
  NS_IMETHOD RespectDisplayPortSuppression(bool aEnabled) = 0;

  /* void forceReflowInterrupt (); */
  NS_IMETHOD ForceReflowInterrupt(void) = 0;

  /* void terminateGPUProcess (); */
  NS_IMETHOD TerminateGPUProcess(void) = 0;

  /* readonly attribute int32_t gpuProcessPid; */
  NS_IMETHOD GetGpuProcessPid(int32_t *aGpuProcessPid) = 0;

  /* void addManuallyManagedState (in Element element, in AString state); */
  NS_IMETHOD AddManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) = 0;

  /* void removeManuallyManagedState (in Element element, in AString state); */
  NS_IMETHOD RemoveManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) = 0;

  /* int64_t getStorageUsage (in Storage aStorage); */
  NS_IMETHOD GetStorageUsage(mozilla::dom::Storage *aStorage, int64_t *_retval) = 0;

  /* long getDirectionFromText (in AString aString); */
  NS_IMETHOD GetDirectionFromText(const nsAString& aString, int32_t *_retval) = 0;

  /* void ensureDirtyRootFrame (); */
  NS_IMETHOD EnsureDirtyRootFrame(void) = 0;

  /* void wrCapture (); */
  NS_IMETHOD WrCapture(void) = 0;

  /* void wrToggleCaptureSequence (); */
  NS_IMETHOD WrToggleCaptureSequence(void) = 0;

  /* Promise setCompositionRecording (in boolean aValue); */
  NS_IMETHOD SetCompositionRecording(bool aValue, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise startCompositionRecording (); */
  NS_IMETHOD StartCompositionRecording(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise stopCompositionRecording (in boolean aWriteToDisk); */
  NS_IMETHOD StopCompositionRecording(bool aWriteToDisk, ::mozilla::dom::Promise * * _retval) = 0;

  /* bool isCssPropertyRecordedInUseCounter (in ACString aProperty); */
  NS_IMETHOD IsCssPropertyRecordedInUseCounter(const nsACString& aProperty, bool *_retval) = 0;

  /* void resetMobileViewportManager (); */
  NS_IMETHOD ResetMobileViewportManager(void) = 0;

  /* attribute ACString systemFont; */
  NS_IMETHOD GetSystemFont(nsACString& aSystemFont) = 0;
  NS_IMETHOD SetSystemFont(const nsACString& aSystemFont) = 0;

  /* readonly attribute unsigned long long paintCount; */
  NS_IMETHOD GetPaintCount(uint64_t *aPaintCount) = 0;

  enum {
    DEFAULT_MOUSE_POINTER_ID = 0,
    DEFAULT_PEN_POINTER_ID = 1,
    DEFAULT_TOUCH_POINTER_ID = 2,
    MOUSE_BUTTON_LEFT_BUTTON = 0,
    MOUSE_BUTTON_MIDDLE_BUTTON = 1,
    MOUSE_BUTTON_RIGHT_BUTTON = 2,
    MOUSE_BUTTONS_NO_BUTTON = 0,
    MOUSE_BUTTONS_LEFT_BUTTON = 1,
    MOUSE_BUTTONS_RIGHT_BUTTON = 2,
    MOUSE_BUTTONS_MIDDLE_BUTTON = 4,
    MOUSE_BUTTONS_4TH_BUTTON = 8,
    MOUSE_BUTTONS_5TH_BUTTON = 16,
    MOUSE_BUTTONS_NOT_SPECIFIED = -1,
    DIRECTION_LTR = 0,
    DIRECTION_RTL = 1,
    DIRECTION_NOT_SET = 2
  };

  /* void syncFlushCompositor (); */
  NS_IMETHOD SyncFlushCompositor(void) = 0;

  /* unsigned long long getLayersId (); */
  NS_IMETHOD GetLayersId(uint64_t *_retval) = 0;

  /* readonly attribute bool usesOverlayScrollbars; */
  NS_IMETHOD GetUsesOverlayScrollbars(bool *aUsesOverlayScrollbars) = 0;

  /* readonly attribute bool effectivelyThrottlesFrameRequests; */
  NS_IMETHOD GetEffectivelyThrottlesFrameRequests(bool *aEffectivelyThrottlesFrameRequests) = 0;

  /* readonly attribute AString webrtcRawDeviceId; */
  NS_IMETHOD GetWebrtcRawDeviceId(nsAString& aWebrtcRawDeviceId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMWindowUtils, NS_IDOMWINDOWUTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMWINDOWUTILS \
  NS_IMETHOD GetImageAnimationMode(uint16_t *aImageAnimationMode) override; \
  NS_IMETHOD SetImageAnimationMode(uint16_t aImageAnimationMode) override; \
  NS_IMETHOD GetDocCharsetIsForced(bool *aDocCharsetIsForced) override; \
  NS_IMETHOD GetPhysicalMillimeterInCSSPixels(float *aPhysicalMillimeterInCSSPixels) override; \
  NS_IMETHOD GetDocumentMetadata(const nsAString& aName, nsAString& _retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateLayerTree(void) override; \
  NS_IMETHOD GetLastTransactionId(uint64_t *aLastTransactionId) override; \
  NS_IMETHOD GetViewportInfo(uint32_t aDisplayWidth, uint32_t aDisplayHeight, double *aDefaultZoom, bool *aAllowZoom, double *aMinZoom, double *aMaxZoom, uint32_t *aWidth, uint32_t *aHeight, bool *aAutoSize) override; \
  NS_IMETHOD GetViewportFitInfo(nsAString& _retval) override; \
  NS_IMETHOD GetContentViewerSize(uint32_t *aDisplayWidth, uint32_t *aDisplayHeight) override; \
  NS_IMETHOD SetDisplayPortForElement(float aXPx, float aYPx, float aWidthPx, float aHeightPx, mozilla::dom::Element *aElement, uint32_t aPriority) override; \
  NS_IMETHOD SetDisplayPortMarginsForElement(float aLeftMargin, float aTopMargin, float aRightMargin, float aBottomMargin, mozilla::dom::Element *aElement, uint32_t aPriority) override; \
  NS_IMETHOD SetDisplayPortBaseForElement(int32_t aX, int32_t aY, int32_t aWidth, int32_t aHeight, mozilla::dom::Element *aElement) override; \
  NS_IMETHOD GetScrollbarSizes(mozilla::dom::Element *aElement, uint32_t *aVerticalScrollbarWidth, uint32_t *aHorizontalScrollbarHeight) override; \
  NS_IMETHOD SetResolutionAndScaleTo(float aResolution) override; \
  NS_IMETHOD GetResolution(float *_retval) override; \
  NS_IMETHOD SetRestoreResolution(float aResolution, uint32_t aDisplayWidth, uint32_t aDisplayHeight) override; \
  NS_IMETHOD GetIsFirstPaint(bool *aIsFirstPaint) override; \
  NS_IMETHOD SetIsFirstPaint(bool aIsFirstPaint) override; \
  NS_IMETHOD GetPresShellId(uint32_t *_retval) override; \
  NS_IMETHOD IsCORSSafelistedRequestHeader(const nsACString& name, const nsACString& value, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEvent(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEvent(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEventToWindow(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEventToWindow(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override; \
  NS_IMETHOD SendWheelEvent(float aX, float aY, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aDeltaMode, int32_t aModifiers, int32_t aLineOrPageDeltaX, int32_t aLineOrPageDeltaY, uint32_t aOptions) override; \
  NS_IMETHOD SendNativeKeyEvent(int32_t aNativeKeyboardLayout, int32_t aNativeKeyCode, int32_t aModifierFlags, const nsAString& aCharacters, const nsAString& aUnmodifiedCharacters, nsIObserver *aObserver) override; \
  NS_IMETHOD SendNativeMouseEvent(int32_t aScreenX, int32_t aScreenY, int32_t aNativeMessage, int32_t aModifierFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override; \
  NS_IMETHOD SendNativeMouseMove(int32_t aScreenX, int32_t aScreenY, mozilla::dom::Element *aElement, nsIObserver *aObserver) override; \
  NS_IMETHOD SuppressAnimation(bool aSuppress) override; \
  NS_IMETHOD SendNativeMouseScrollEvent(int32_t aScreenX, int32_t aScreenY, uint32_t aNativeMessage, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aModifierFlags, uint32_t aAdditionalFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override; \
  NS_IMETHOD SendNativeTouchPoint(uint32_t aPointerId, uint32_t aTouchState, int32_t aScreenX, int32_t aScreenY, double aPressure, uint32_t aOrientation, nsIObserver *aObserver) override; \
  NS_IMETHOD SendNativeTouchpadPinch(uint32_t aEventPhase, float aScale, int32_t aScreenX, int32_t aScreenY, int32_t aModifierFlags) override; \
  NS_IMETHOD SendNativeTouchTap(int32_t aScreenX, int32_t aScreenY, bool aLongTap, nsIObserver *aObserver) override; \
  NS_IMETHOD ClearNativeTouchSequence(nsIObserver *aObserver) override; \
  NS_IMETHOD ClearSharedStyleSheetCache(void) override; \
  NS_IMETHOD GetParsedStyleSheets(uint32_t *aParsedStyleSheets) override; \
  NS_IMETHOD ActivateNativeMenuItemAt(const nsAString& indexString) override; \
  NS_IMETHOD ForceUpdateNativeMenuAt(const nsAString& indexString) override; \
  NS_IMETHOD GetSelectionAsPlaintext(nsAString& _retval) override; \
  NS_IMETHOD GarbageCollect(nsICycleCollectorListener *aListener) override; \
  NS_IMETHOD CycleCollect(nsICycleCollectorListener *aListener) override; \
  NS_IMETHOD RunNextCollectorTimer(void) override; \
  NS_IMETHOD SendSimpleGestureEvent(const nsAString& aType, float aX, float aY, uint32_t aDirection, double aDelta, int32_t aModifiers, uint32_t aClickCount) override; \
  NS_IMETHOD ElementFromPoint(float aX, float aY, bool aIgnoreRootScrollFrame, bool aFlushLayout, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD NodesFromRect(float aX, float aY, float aTopSize, float aRightSize, float aBottomSize, float aLeftSize, bool aIgnoreRootScrollFrame, bool aFlushLayout, bool aOnlyVisible, float aTransparencyThreshold, nsINodeList **_retval) override; \
  NS_IMETHOD GetTranslationNodes(nsINode *aRoot, nsITranslationNodeList **_retval) override; \
  NS_IMETHOD CompareCanvases(nsISupports *aCanvas1, nsISupports *aCanvas2, uint32_t *aMaxDifference, uint32_t *_retval) override; \
  NS_IMETHOD GetIsMozAfterPaintPending(bool *aIsMozAfterPaintPending) override; \
  NS_IMETHOD GetIsInputTaskManagerSuspended(bool *aIsInputTaskManagerSuspended) override; \
  NS_IMETHOD SuppressEventHandling(bool aSuppress) override; \
  NS_IMETHOD DisableNonTestMouseEvents(bool aDisable) override; \
  NS_IMETHOD GetScrollXY(bool aFlushLayout, int32_t *aScrollX, int32_t *aScrollY) override; \
  NS_IMETHOD GetScrollXYFloat(bool aFlushLayout, float *aScrollX, float *aScrollY) override; \
  NS_IMETHOD GetScrollbarSize(bool aFlushLayout, int32_t *aWidth, int32_t *aHeight) override; \
  NS_IMETHOD GetBoundsWithoutFlushing(mozilla::dom::Element *aElement, mozilla::dom::DOMRect **_retval) override; \
  NS_IMETHOD ScrollToVisual(float aOffsetX, float aOffsetY, int32_t aUpdateType, int32_t aScrollMode) override; \
  NS_IMETHOD GetVisualViewportOffsetRelativeToLayoutViewport(float *aOffsetX, float *aOffsetY) override; \
  NS_IMETHOD GetVisualViewportOffset(int32_t *aOffsetX, int32_t *aOffsetY) override; \
  NS_IMETHOD TransformRectLayoutToVisual(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override; \
  NS_IMETHOD ToScreenRect(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDynamicToolbarMaxHeight(uint32_t aHeightInScreen) override; \
  NS_IMETHOD NeedsFlush(int32_t aFlushtype, bool *_retval) override; \
  NS_IMETHOD FlushLayoutWithoutThrottledAnimations(void) override; \
  NS_IMETHOD GetRootBounds(mozilla::dom::DOMRect **_retval) override; \
  NS_IMETHOD GetIMEIsOpen(bool *aIMEIsOpen) override; \
  NS_IMETHOD GetIMEStatus(uint32_t *aIMEStatus) override; \
  NS_IMETHOD GetScreenPixelsPerCSSPixel(float *aScreenPixelsPerCSSPixel) override; \
  NS_IMETHOD GetScreenPixelsPerCSSPixelNoOverride(float *aScreenPixelsPerCSSPixelNoOverride) override; \
  NS_IMETHOD GetFullZoom(float *aFullZoom) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DispatchDOMEventViaPresShellForTesting(nsINode *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override; \
  NS_IMETHOD DispatchEventToChromeOnly(mozilla::dom::EventTarget *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override; \
  NS_IMETHOD GetClassName(JS::HandleValue aObject, JSContext* cx, char * *_retval) override; \
  NS_IMETHOD SendContentCommandEvent(const nsAString& aType, nsITransferable *aTransferable) override; \
  NS_IMETHOD SendQueryContentEvent(uint32_t aType, int64_t aOffset, uint32_t aLength, int32_t aX, int32_t aY, uint32_t aAdditionalFlags, nsIQueryContentEventResult **_retval) override; \
  NS_IMETHOD RemoteFrameFullscreenChanged(mozilla::dom::Element *aFrameElement) override; \
  NS_IMETHOD RemoteFrameFullscreenReverted(void) override; \
  NS_IMETHOD HandleFullscreenRequests(bool *_retval) override; \
  NS_IMETHOD ExitFullscreen(void) override; \
  NS_IMETHOD SendSelectionSetEvent(uint32_t aOffset, uint32_t aLength, uint32_t aAdditionalFlags, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAtPoint(float aX, float aY, uint32_t aSelectBehavior, bool *_retval) override; \
  NS_IMETHOD GetVisitedDependentComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aPropertyName, nsAString& _retval) override; \
  NS_IMETHOD GetDeprecatedOuterWindowID(uint64_t *aDeprecatedOuterWindowID) override; \
  NS_IMETHOD EnterModalState(void) override; \
  NS_IMETHOD LeaveModalState(void) override; \
  NS_IMETHOD IsInModalState(bool *_retval) override; \
  NS_IMETHOD GetDesktopModeViewport(bool *aDesktopModeViewport) override; \
  NS_IMETHOD SetDesktopModeViewport(bool aDesktopModeViewport) override; \
  NS_IMETHOD SuspendTimeouts(void) override; \
  NS_IMETHOD ResumeTimeouts(void) override; \
  NS_IMETHOD GetLayerManagerType(nsAString& aLayerManagerType) override; \
  NS_IMETHOD GetLayerManagerRemote(bool *aLayerManagerRemote) override; \
  NS_IMETHOD GetUsingAdvancedLayers(bool *aUsingAdvancedLayers) override; \
  NS_IMETHOD GetIsWebRenderRequested(bool *aIsWebRenderRequested) override; \
  NS_IMETHOD GetCurrentAudioBackend(nsAString& aCurrentAudioBackend) override; \
  NS_IMETHOD GetCurrentMaxAudioChannels(uint32_t *aCurrentMaxAudioChannels) override; \
  NS_IMETHOD DefaultDevicesRoundTripLatency(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetCurrentPreferredSampleRate(uint32_t *aCurrentPreferredSampleRate) override; \
  NS_IMETHOD AudioDevices(uint16_t aSide, nsIArray **_retval) override; \
  NS_IMETHOD StartFrameTimeRecording(uint32_t *startIndex) override; \
  NS_IMETHOD StopFrameTimeRecording(uint32_t startIndex, nsTArray<float >& _retval) override; \
  NS_IMETHOD GetDisplayDPI(float *aDisplayDPI) override; \
  NS_IMETHOD AdvanceTimeAndRefresh(int64_t aMilliseconds) override; \
  NS_IMETHOD RestoreNormalRefresh(void) override; \
  NS_IMETHOD GetIsTestControllingRefreshes(bool *aIsTestControllingRefreshes) override; \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override; \
  NS_IMETHOD SetAsyncScrollOffset(mozilla::dom::Element *aElement, float aX, float aY) override; \
  NS_IMETHOD SetAsyncZoom(mozilla::dom::Element *aRootElement, float aValue) override; \
  NS_IMETHOD FlushApzRepaints(bool *_retval) override; \
  NS_IMETHOD DisableApzForElement(mozilla::dom::Element *aElement) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ZoomToFocusedInput(void) override; \
  NS_IMETHOD ComputeAnimationDistance(mozilla::dom::Element *element, const nsAString& property, const nsAString& value1, const nsAString& value2, double *_retval) override; \
  NS_IMETHOD GetUnanimatedComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aProperty, int32_t aFlushType, nsAString& _retval) override; \
  NS_IMETHOD GetFocusedInputType(nsAString& aFocusedInputType) override; \
  NS_IMETHOD GetFocusedActionHint(nsAString& aFocusedActionHint) override; \
  NS_IMETHOD GetFocusedInputMode(nsAString& aFocusedInputMode) override; \
  NS_IMETHOD GetFocusedAutocapitalize(nsAString& aFocusedAutocapitalize) override; \
  NS_IMETHOD GetViewId(mozilla::dom::Element *aElement, nsViewID *_retval) override; \
  NS_IMETHOD LeafLayersPartitionWindow(bool *_retval) override; \
  NS_IMETHOD CheckAndClearPaintedState(mozilla::dom::Element *aElement, bool *_retval) override; \
  NS_IMETHOD CheckAndClearDisplayListState(mozilla::dom::Element *aElement, bool *_retval) override; \
  NS_IMETHOD IsPartOfOpaqueLayer(mozilla::dom::Element *aElement, bool *_retval) override; \
  NS_IMETHOD NumberOfAssignedPaintedLayers(const nsTArray<RefPtr<mozilla::dom::Element>>& aElements, uint32_t *_retval) override; \
  NS_IMETHOD GetFileId(JS::HandleValue aFile, JSContext* cx, int64_t *_retval) override; \
  NS_IMETHOD GetFilePath(JS::HandleValue aFile, JSContext* cx, nsAString& _retval) override; \
  NS_IMETHOD GetFileReferences(const nsAString& aDatabaseName, int64_t aId, JS::HandleValue aOptions, int32_t *aRefCnt, int32_t *aDBRefCnt, JSContext* cx, bool *_retval) override; \
  NS_IMETHOD FlushPendingFileDeletions(void) override; \
  NS_IMETHOD StartPCCountProfiling(JSContext* cx) override; \
  NS_IMETHOD StopPCCountProfiling(JSContext* cx) override; \
  NS_IMETHOD PurgePCCounts(JSContext* cx) override; \
  NS_IMETHOD GetPCCountScriptCount(JSContext* cx, int32_t *_retval) override; \
  NS_IMETHOD GetPCCountScriptSummary(int32_t script, JSContext* cx, nsAString& _retval) override; \
  NS_IMETHOD GetPCCountScriptContents(int32_t script, JSContext* cx, nsAString& _retval) override; \
  NS_IMETHOD GetPaintingSuppressed(bool *aPaintingSuppressed) override; \
  NS_IMETHOD GetPlugins(JSContext* cx, JS::MutableHandleValue aPlugins) override; \
  NS_IMETHOD SetVisualViewportSize(float aWidth, float aHeight) override; \
  NS_IMETHOD DisableDialogs(void) override; \
  NS_IMETHOD EnableDialogs(void) override; \
  NS_IMETHOD AreDialogsEnabled(bool *_retval) override; \
  NS_IMETHOD LoadSheet(nsIURI *sheetURI, uint32_t type) override; \
  NS_IMETHOD LoadSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override; \
  NS_IMETHOD AddSheet(nsIPreloadedStyleSheet *sheet, uint32_t type) override; \
  NS_IMETHOD RemoveSheet(nsIURI *sheetURI, uint32_t type) override; \
  NS_IMETHOD RemoveSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override; \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override; \
  NS_IMETHOD GetMillisSinceLastUserInput(double *aMillisSinceLastUserInput) override; \
  NS_IMETHOD AllowScriptsToClose(void) override; \
  NS_IMETHOD GetIsParentWindowMainWidgetVisible(bool *aIsParentWindowMainWidgetVisible) override; \
  NS_IMETHOD IsNodeDisabledForEvents(nsINode *aNode, bool *_retval) override; \
  NS_IMETHOD GetPaintFlashing(bool *aPaintFlashing) override; \
  NS_IMETHOD SetPaintFlashing(bool aPaintFlashing) override; \
  NS_IMETHOD GetOMTAStyle(mozilla::dom::Element *aElement, const nsAString& aProperty, const nsAString& aPseudoElement, nsAString& _retval) override; \
  NS_IMETHOD GetOMTCTransform(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, nsAString& _retval) override; \
  NS_IMETHOD IsAnimationInPendingTracker(mozilla::dom::Animation *aAnimation, bool *_retval) override; \
  NS_IMETHOD SetHandlingUserInput(bool aHandlingInput, nsIJSRAIIHelper **_retval) override; \
  NS_IMETHOD GetContentAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetCompositorAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD PostRestyleSelfEvent(mozilla::dom::Element *aElement) override; \
  NS_IMETHOD XpconnectArgument(nsISupports *aObj) override; \
  NS_IMETHOD AskPermission(nsIContentPermissionRequest *aRequest) override; \
  NS_IMETHOD GetRestyleGeneration(uint64_t *aRestyleGeneration) override; \
  NS_IMETHOD GetFramesConstructed(uint64_t *aFramesConstructed) override; \
  NS_IMETHOD GetFramesReflowed(uint64_t *aFramesReflowed) override; \
  NS_IMETHOD SetChromeMargin(int32_t aTop, int32_t aRight, int32_t aBottom, int32_t aLeft) override; \
  NS_IMETHOD GetFrameUniformityTestData(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD EnterChaosMode(void) override; \
  NS_IMETHOD LeaveChaosMode(void) override; \
  NS_IMETHOD TriggerDeviceReset(void) override; \
  NS_IMETHOD HasRuleProcessorUsedByMultipleStyleSets(uint32_t aSheetType, bool *_retval) override; \
  NS_IMETHOD RespectDisplayPortSuppression(bool aEnabled) override; \
  NS_IMETHOD ForceReflowInterrupt(void) override; \
  NS_IMETHOD TerminateGPUProcess(void) override; \
  NS_IMETHOD GetGpuProcessPid(int32_t *aGpuProcessPid) override; \
  NS_IMETHOD AddManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override; \
  NS_IMETHOD RemoveManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override; \
  NS_IMETHOD GetStorageUsage(mozilla::dom::Storage *aStorage, int64_t *_retval) override; \
  NS_IMETHOD GetDirectionFromText(const nsAString& aString, int32_t *_retval) override; \
  NS_IMETHOD EnsureDirtyRootFrame(void) override; \
  NS_IMETHOD WrCapture(void) override; \
  NS_IMETHOD WrToggleCaptureSequence(void) override; \
  NS_IMETHOD SetCompositionRecording(bool aValue, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD StartCompositionRecording(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD StopCompositionRecording(bool aWriteToDisk, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD IsCssPropertyRecordedInUseCounter(const nsACString& aProperty, bool *_retval) override; \
  NS_IMETHOD ResetMobileViewportManager(void) override; \
  NS_IMETHOD GetSystemFont(nsACString& aSystemFont) override; \
  NS_IMETHOD SetSystemFont(const nsACString& aSystemFont) override; \
  NS_IMETHOD GetPaintCount(uint64_t *aPaintCount) override; \
  NS_IMETHOD SyncFlushCompositor(void) override; \
  NS_IMETHOD GetLayersId(uint64_t *_retval) override; \
  NS_IMETHOD GetUsesOverlayScrollbars(bool *aUsesOverlayScrollbars) override; \
  NS_IMETHOD GetEffectivelyThrottlesFrameRequests(bool *aEffectivelyThrottlesFrameRequests) override; \
  NS_IMETHOD GetWebrtcRawDeviceId(nsAString& aWebrtcRawDeviceId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMWINDOWUTILS \
  nsresult GetImageAnimationMode(uint16_t *aImageAnimationMode); \
  nsresult SetImageAnimationMode(uint16_t aImageAnimationMode); \
  nsresult GetDocCharsetIsForced(bool *aDocCharsetIsForced); \
  nsresult GetPhysicalMillimeterInCSSPixels(float *aPhysicalMillimeterInCSSPixels); \
  nsresult GetDocumentMetadata(const nsAString& aName, nsAString& _retval); \
  MOZ_CAN_RUN_SCRIPT nsresult UpdateLayerTree(void); \
  nsresult GetLastTransactionId(uint64_t *aLastTransactionId); \
  nsresult GetViewportInfo(uint32_t aDisplayWidth, uint32_t aDisplayHeight, double *aDefaultZoom, bool *aAllowZoom, double *aMinZoom, double *aMaxZoom, uint32_t *aWidth, uint32_t *aHeight, bool *aAutoSize); \
  nsresult GetViewportFitInfo(nsAString& _retval); \
  nsresult GetContentViewerSize(uint32_t *aDisplayWidth, uint32_t *aDisplayHeight); \
  nsresult SetDisplayPortForElement(float aXPx, float aYPx, float aWidthPx, float aHeightPx, mozilla::dom::Element *aElement, uint32_t aPriority); \
  nsresult SetDisplayPortMarginsForElement(float aLeftMargin, float aTopMargin, float aRightMargin, float aBottomMargin, mozilla::dom::Element *aElement, uint32_t aPriority); \
  nsresult SetDisplayPortBaseForElement(int32_t aX, int32_t aY, int32_t aWidth, int32_t aHeight, mozilla::dom::Element *aElement); \
  nsresult GetScrollbarSizes(mozilla::dom::Element *aElement, uint32_t *aVerticalScrollbarWidth, uint32_t *aHorizontalScrollbarHeight); \
  nsresult SetResolutionAndScaleTo(float aResolution); \
  nsresult GetResolution(float *_retval); \
  nsresult SetRestoreResolution(float aResolution, uint32_t aDisplayWidth, uint32_t aDisplayHeight); \
  nsresult GetIsFirstPaint(bool *aIsFirstPaint); \
  nsresult SetIsFirstPaint(bool aIsFirstPaint); \
  nsresult GetPresShellId(uint32_t *_retval); \
  nsresult IsCORSSafelistedRequestHeader(const nsACString& name, const nsACString& value, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SendMouseEvent(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SendTouchEvent(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SendMouseEventToWindow(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc); \
  MOZ_CAN_RUN_SCRIPT nsresult SendTouchEventToWindow(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval); \
  nsresult SendWheelEvent(float aX, float aY, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aDeltaMode, int32_t aModifiers, int32_t aLineOrPageDeltaX, int32_t aLineOrPageDeltaY, uint32_t aOptions); \
  nsresult SendNativeKeyEvent(int32_t aNativeKeyboardLayout, int32_t aNativeKeyCode, int32_t aModifierFlags, const nsAString& aCharacters, const nsAString& aUnmodifiedCharacters, nsIObserver *aObserver); \
  nsresult SendNativeMouseEvent(int32_t aScreenX, int32_t aScreenY, int32_t aNativeMessage, int32_t aModifierFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver); \
  nsresult SendNativeMouseMove(int32_t aScreenX, int32_t aScreenY, mozilla::dom::Element *aElement, nsIObserver *aObserver); \
  nsresult SuppressAnimation(bool aSuppress); \
  nsresult SendNativeMouseScrollEvent(int32_t aScreenX, int32_t aScreenY, uint32_t aNativeMessage, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aModifierFlags, uint32_t aAdditionalFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver); \
  nsresult SendNativeTouchPoint(uint32_t aPointerId, uint32_t aTouchState, int32_t aScreenX, int32_t aScreenY, double aPressure, uint32_t aOrientation, nsIObserver *aObserver); \
  nsresult SendNativeTouchpadPinch(uint32_t aEventPhase, float aScale, int32_t aScreenX, int32_t aScreenY, int32_t aModifierFlags); \
  nsresult SendNativeTouchTap(int32_t aScreenX, int32_t aScreenY, bool aLongTap, nsIObserver *aObserver); \
  nsresult ClearNativeTouchSequence(nsIObserver *aObserver); \
  nsresult ClearSharedStyleSheetCache(void); \
  nsresult GetParsedStyleSheets(uint32_t *aParsedStyleSheets); \
  nsresult ActivateNativeMenuItemAt(const nsAString& indexString); \
  nsresult ForceUpdateNativeMenuAt(const nsAString& indexString); \
  nsresult GetSelectionAsPlaintext(nsAString& _retval); \
  nsresult GarbageCollect(nsICycleCollectorListener *aListener); \
  nsresult CycleCollect(nsICycleCollectorListener *aListener); \
  nsresult RunNextCollectorTimer(void); \
  nsresult SendSimpleGestureEvent(const nsAString& aType, float aX, float aY, uint32_t aDirection, double aDelta, int32_t aModifiers, uint32_t aClickCount); \
  nsresult ElementFromPoint(float aX, float aY, bool aIgnoreRootScrollFrame, bool aFlushLayout, mozilla::dom::Element **_retval); \
  nsresult NodesFromRect(float aX, float aY, float aTopSize, float aRightSize, float aBottomSize, float aLeftSize, bool aIgnoreRootScrollFrame, bool aFlushLayout, bool aOnlyVisible, float aTransparencyThreshold, nsINodeList **_retval); \
  nsresult GetTranslationNodes(nsINode *aRoot, nsITranslationNodeList **_retval); \
  nsresult CompareCanvases(nsISupports *aCanvas1, nsISupports *aCanvas2, uint32_t *aMaxDifference, uint32_t *_retval); \
  nsresult GetIsMozAfterPaintPending(bool *aIsMozAfterPaintPending); \
  nsresult GetIsInputTaskManagerSuspended(bool *aIsInputTaskManagerSuspended); \
  nsresult SuppressEventHandling(bool aSuppress); \
  nsresult DisableNonTestMouseEvents(bool aDisable); \
  nsresult GetScrollXY(bool aFlushLayout, int32_t *aScrollX, int32_t *aScrollY); \
  nsresult GetScrollXYFloat(bool aFlushLayout, float *aScrollX, float *aScrollY); \
  nsresult GetScrollbarSize(bool aFlushLayout, int32_t *aWidth, int32_t *aHeight); \
  nsresult GetBoundsWithoutFlushing(mozilla::dom::Element *aElement, mozilla::dom::DOMRect **_retval); \
  nsresult ScrollToVisual(float aOffsetX, float aOffsetY, int32_t aUpdateType, int32_t aScrollMode); \
  nsresult GetVisualViewportOffsetRelativeToLayoutViewport(float *aOffsetX, float *aOffsetY); \
  nsresult GetVisualViewportOffset(int32_t *aOffsetX, int32_t *aOffsetY); \
  nsresult TransformRectLayoutToVisual(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval); \
  nsresult ToScreenRect(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SetDynamicToolbarMaxHeight(uint32_t aHeightInScreen); \
  nsresult NeedsFlush(int32_t aFlushtype, bool *_retval); \
  nsresult FlushLayoutWithoutThrottledAnimations(void); \
  nsresult GetRootBounds(mozilla::dom::DOMRect **_retval); \
  nsresult GetIMEIsOpen(bool *aIMEIsOpen); \
  nsresult GetIMEStatus(uint32_t *aIMEStatus); \
  nsresult GetScreenPixelsPerCSSPixel(float *aScreenPixelsPerCSSPixel); \
  nsresult GetScreenPixelsPerCSSPixelNoOverride(float *aScreenPixelsPerCSSPixelNoOverride); \
  nsresult GetFullZoom(float *aFullZoom); \
  MOZ_CAN_RUN_SCRIPT nsresult DispatchDOMEventViaPresShellForTesting(nsINode *aTarget, mozilla::dom::Event *aEvent, bool *_retval); \
  nsresult DispatchEventToChromeOnly(mozilla::dom::EventTarget *aTarget, mozilla::dom::Event *aEvent, bool *_retval); \
  nsresult GetClassName(JS::HandleValue aObject, JSContext* cx, char * *_retval); \
  nsresult SendContentCommandEvent(const nsAString& aType, nsITransferable *aTransferable); \
  nsresult SendQueryContentEvent(uint32_t aType, int64_t aOffset, uint32_t aLength, int32_t aX, int32_t aY, uint32_t aAdditionalFlags, nsIQueryContentEventResult **_retval); \
  nsresult RemoteFrameFullscreenChanged(mozilla::dom::Element *aFrameElement); \
  nsresult RemoteFrameFullscreenReverted(void); \
  nsresult HandleFullscreenRequests(bool *_retval); \
  nsresult ExitFullscreen(void); \
  nsresult SendSelectionSetEvent(uint32_t aOffset, uint32_t aLength, uint32_t aAdditionalFlags, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SelectAtPoint(float aX, float aY, uint32_t aSelectBehavior, bool *_retval); \
  nsresult GetVisitedDependentComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aPropertyName, nsAString& _retval); \
  nsresult GetDeprecatedOuterWindowID(uint64_t *aDeprecatedOuterWindowID); \
  nsresult EnterModalState(void); \
  nsresult LeaveModalState(void); \
  nsresult IsInModalState(bool *_retval); \
  nsresult GetDesktopModeViewport(bool *aDesktopModeViewport); \
  nsresult SetDesktopModeViewport(bool aDesktopModeViewport); \
  nsresult SuspendTimeouts(void); \
  nsresult ResumeTimeouts(void); \
  nsresult GetLayerManagerType(nsAString& aLayerManagerType); \
  nsresult GetLayerManagerRemote(bool *aLayerManagerRemote); \
  nsresult GetUsingAdvancedLayers(bool *aUsingAdvancedLayers); \
  nsresult GetIsWebRenderRequested(bool *aIsWebRenderRequested); \
  nsresult GetCurrentAudioBackend(nsAString& aCurrentAudioBackend); \
  nsresult GetCurrentMaxAudioChannels(uint32_t *aCurrentMaxAudioChannels); \
  nsresult DefaultDevicesRoundTripLatency(::mozilla::dom::Promise * * _retval); \
  nsresult GetCurrentPreferredSampleRate(uint32_t *aCurrentPreferredSampleRate); \
  nsresult AudioDevices(uint16_t aSide, nsIArray **_retval); \
  nsresult StartFrameTimeRecording(uint32_t *startIndex); \
  nsresult StopFrameTimeRecording(uint32_t startIndex, nsTArray<float >& _retval); \
  nsresult GetDisplayDPI(float *aDisplayDPI); \
  nsresult AdvanceTimeAndRefresh(int64_t aMilliseconds); \
  nsresult RestoreNormalRefresh(void); \
  nsresult GetIsTestControllingRefreshes(bool *aIsTestControllingRefreshes); \
  nsresult GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled); \
  nsresult SetAsyncScrollOffset(mozilla::dom::Element *aElement, float aX, float aY); \
  nsresult SetAsyncZoom(mozilla::dom::Element *aRootElement, float aValue); \
  nsresult FlushApzRepaints(bool *_retval); \
  nsresult DisableApzForElement(mozilla::dom::Element *aElement); \
  MOZ_CAN_RUN_SCRIPT nsresult ZoomToFocusedInput(void); \
  nsresult ComputeAnimationDistance(mozilla::dom::Element *element, const nsAString& property, const nsAString& value1, const nsAString& value2, double *_retval); \
  nsresult GetUnanimatedComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aProperty, int32_t aFlushType, nsAString& _retval); \
  nsresult GetFocusedInputType(nsAString& aFocusedInputType); \
  nsresult GetFocusedActionHint(nsAString& aFocusedActionHint); \
  nsresult GetFocusedInputMode(nsAString& aFocusedInputMode); \
  nsresult GetFocusedAutocapitalize(nsAString& aFocusedAutocapitalize); \
  nsresult GetViewId(mozilla::dom::Element *aElement, nsViewID *_retval); \
  nsresult LeafLayersPartitionWindow(bool *_retval); \
  nsresult CheckAndClearPaintedState(mozilla::dom::Element *aElement, bool *_retval); \
  nsresult CheckAndClearDisplayListState(mozilla::dom::Element *aElement, bool *_retval); \
  nsresult IsPartOfOpaqueLayer(mozilla::dom::Element *aElement, bool *_retval); \
  nsresult NumberOfAssignedPaintedLayers(const nsTArray<RefPtr<mozilla::dom::Element>>& aElements, uint32_t *_retval); \
  nsresult GetFileId(JS::HandleValue aFile, JSContext* cx, int64_t *_retval); \
  nsresult GetFilePath(JS::HandleValue aFile, JSContext* cx, nsAString& _retval); \
  nsresult GetFileReferences(const nsAString& aDatabaseName, int64_t aId, JS::HandleValue aOptions, int32_t *aRefCnt, int32_t *aDBRefCnt, JSContext* cx, bool *_retval); \
  nsresult FlushPendingFileDeletions(void); \
  nsresult StartPCCountProfiling(JSContext* cx); \
  nsresult StopPCCountProfiling(JSContext* cx); \
  nsresult PurgePCCounts(JSContext* cx); \
  nsresult GetPCCountScriptCount(JSContext* cx, int32_t *_retval); \
  nsresult GetPCCountScriptSummary(int32_t script, JSContext* cx, nsAString& _retval); \
  nsresult GetPCCountScriptContents(int32_t script, JSContext* cx, nsAString& _retval); \
  nsresult GetPaintingSuppressed(bool *aPaintingSuppressed); \
  nsresult GetPlugins(JSContext* cx, JS::MutableHandleValue aPlugins); \
  nsresult SetVisualViewportSize(float aWidth, float aHeight); \
  nsresult DisableDialogs(void); \
  nsresult EnableDialogs(void); \
  nsresult AreDialogsEnabled(bool *_retval); \
  nsresult LoadSheet(nsIURI *sheetURI, uint32_t type); \
  nsresult LoadSheetUsingURIString(const nsACString& sheetURI, uint32_t type); \
  nsresult AddSheet(nsIPreloadedStyleSheet *sheet, uint32_t type); \
  nsresult RemoveSheet(nsIURI *sheetURI, uint32_t type); \
  nsresult RemoveSheetUsingURIString(const nsACString& sheetURI, uint32_t type); \
  nsresult GetIsHandlingUserInput(bool *aIsHandlingUserInput); \
  nsresult GetMillisSinceLastUserInput(double *aMillisSinceLastUserInput); \
  nsresult AllowScriptsToClose(void); \
  nsresult GetIsParentWindowMainWidgetVisible(bool *aIsParentWindowMainWidgetVisible); \
  nsresult IsNodeDisabledForEvents(nsINode *aNode, bool *_retval); \
  nsresult GetPaintFlashing(bool *aPaintFlashing); \
  nsresult SetPaintFlashing(bool aPaintFlashing); \
  nsresult GetOMTAStyle(mozilla::dom::Element *aElement, const nsAString& aProperty, const nsAString& aPseudoElement, nsAString& _retval); \
  nsresult GetOMTCTransform(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, nsAString& _retval); \
  nsresult IsAnimationInPendingTracker(mozilla::dom::Animation *aAnimation, bool *_retval); \
  nsresult SetHandlingUserInput(bool aHandlingInput, nsIJSRAIIHelper **_retval); \
  nsresult GetContentAPZTestData(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult GetCompositorAPZTestData(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult PostRestyleSelfEvent(mozilla::dom::Element *aElement); \
  nsresult XpconnectArgument(nsISupports *aObj); \
  nsresult AskPermission(nsIContentPermissionRequest *aRequest); \
  nsresult GetRestyleGeneration(uint64_t *aRestyleGeneration); \
  nsresult GetFramesConstructed(uint64_t *aFramesConstructed); \
  nsresult GetFramesReflowed(uint64_t *aFramesReflowed); \
  nsresult SetChromeMargin(int32_t aTop, int32_t aRight, int32_t aBottom, int32_t aLeft); \
  nsresult GetFrameUniformityTestData(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult EnterChaosMode(void); \
  nsresult LeaveChaosMode(void); \
  nsresult TriggerDeviceReset(void); \
  nsresult HasRuleProcessorUsedByMultipleStyleSets(uint32_t aSheetType, bool *_retval); \
  nsresult RespectDisplayPortSuppression(bool aEnabled); \
  nsresult ForceReflowInterrupt(void); \
  nsresult TerminateGPUProcess(void); \
  nsresult GetGpuProcessPid(int32_t *aGpuProcessPid); \
  nsresult AddManuallyManagedState(mozilla::dom::Element *element, const nsAString& state); \
  nsresult RemoveManuallyManagedState(mozilla::dom::Element *element, const nsAString& state); \
  nsresult GetStorageUsage(mozilla::dom::Storage *aStorage, int64_t *_retval); \
  nsresult GetDirectionFromText(const nsAString& aString, int32_t *_retval); \
  nsresult EnsureDirtyRootFrame(void); \
  nsresult WrCapture(void); \
  nsresult WrToggleCaptureSequence(void); \
  nsresult SetCompositionRecording(bool aValue, ::mozilla::dom::Promise * * _retval); \
  nsresult StartCompositionRecording(::mozilla::dom::Promise * * _retval); \
  nsresult StopCompositionRecording(bool aWriteToDisk, ::mozilla::dom::Promise * * _retval); \
  nsresult IsCssPropertyRecordedInUseCounter(const nsACString& aProperty, bool *_retval); \
  nsresult ResetMobileViewportManager(void); \
  nsresult GetSystemFont(nsACString& aSystemFont); \
  nsresult SetSystemFont(const nsACString& aSystemFont); \
  nsresult GetPaintCount(uint64_t *aPaintCount); \
  nsresult SyncFlushCompositor(void); \
  nsresult GetLayersId(uint64_t *_retval); \
  nsresult GetUsesOverlayScrollbars(bool *aUsesOverlayScrollbars); \
  nsresult GetEffectivelyThrottlesFrameRequests(bool *aEffectivelyThrottlesFrameRequests); \
  nsresult GetWebrtcRawDeviceId(nsAString& aWebrtcRawDeviceId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMWINDOWUTILS(_to) \
  NS_IMETHOD GetImageAnimationMode(uint16_t *aImageAnimationMode) override { return _to GetImageAnimationMode(aImageAnimationMode); } \
  NS_IMETHOD SetImageAnimationMode(uint16_t aImageAnimationMode) override { return _to SetImageAnimationMode(aImageAnimationMode); } \
  NS_IMETHOD GetDocCharsetIsForced(bool *aDocCharsetIsForced) override { return _to GetDocCharsetIsForced(aDocCharsetIsForced); } \
  NS_IMETHOD GetPhysicalMillimeterInCSSPixels(float *aPhysicalMillimeterInCSSPixels) override { return _to GetPhysicalMillimeterInCSSPixels(aPhysicalMillimeterInCSSPixels); } \
  NS_IMETHOD GetDocumentMetadata(const nsAString& aName, nsAString& _retval) override { return _to GetDocumentMetadata(aName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateLayerTree(void) override { return _to UpdateLayerTree(); } \
  NS_IMETHOD GetLastTransactionId(uint64_t *aLastTransactionId) override { return _to GetLastTransactionId(aLastTransactionId); } \
  NS_IMETHOD GetViewportInfo(uint32_t aDisplayWidth, uint32_t aDisplayHeight, double *aDefaultZoom, bool *aAllowZoom, double *aMinZoom, double *aMaxZoom, uint32_t *aWidth, uint32_t *aHeight, bool *aAutoSize) override { return _to GetViewportInfo(aDisplayWidth, aDisplayHeight, aDefaultZoom, aAllowZoom, aMinZoom, aMaxZoom, aWidth, aHeight, aAutoSize); } \
  NS_IMETHOD GetViewportFitInfo(nsAString& _retval) override { return _to GetViewportFitInfo(_retval); } \
  NS_IMETHOD GetContentViewerSize(uint32_t *aDisplayWidth, uint32_t *aDisplayHeight) override { return _to GetContentViewerSize(aDisplayWidth, aDisplayHeight); } \
  NS_IMETHOD SetDisplayPortForElement(float aXPx, float aYPx, float aWidthPx, float aHeightPx, mozilla::dom::Element *aElement, uint32_t aPriority) override { return _to SetDisplayPortForElement(aXPx, aYPx, aWidthPx, aHeightPx, aElement, aPriority); } \
  NS_IMETHOD SetDisplayPortMarginsForElement(float aLeftMargin, float aTopMargin, float aRightMargin, float aBottomMargin, mozilla::dom::Element *aElement, uint32_t aPriority) override { return _to SetDisplayPortMarginsForElement(aLeftMargin, aTopMargin, aRightMargin, aBottomMargin, aElement, aPriority); } \
  NS_IMETHOD SetDisplayPortBaseForElement(int32_t aX, int32_t aY, int32_t aWidth, int32_t aHeight, mozilla::dom::Element *aElement) override { return _to SetDisplayPortBaseForElement(aX, aY, aWidth, aHeight, aElement); } \
  NS_IMETHOD GetScrollbarSizes(mozilla::dom::Element *aElement, uint32_t *aVerticalScrollbarWidth, uint32_t *aHorizontalScrollbarHeight) override { return _to GetScrollbarSizes(aElement, aVerticalScrollbarWidth, aHorizontalScrollbarHeight); } \
  NS_IMETHOD SetResolutionAndScaleTo(float aResolution) override { return _to SetResolutionAndScaleTo(aResolution); } \
  NS_IMETHOD GetResolution(float *_retval) override { return _to GetResolution(_retval); } \
  NS_IMETHOD SetRestoreResolution(float aResolution, uint32_t aDisplayWidth, uint32_t aDisplayHeight) override { return _to SetRestoreResolution(aResolution, aDisplayWidth, aDisplayHeight); } \
  NS_IMETHOD GetIsFirstPaint(bool *aIsFirstPaint) override { return _to GetIsFirstPaint(aIsFirstPaint); } \
  NS_IMETHOD SetIsFirstPaint(bool aIsFirstPaint) override { return _to SetIsFirstPaint(aIsFirstPaint); } \
  NS_IMETHOD GetPresShellId(uint32_t *_retval) override { return _to GetPresShellId(_retval); } \
  NS_IMETHOD IsCORSSafelistedRequestHeader(const nsACString& name, const nsACString& value, bool *_retval) override { return _to IsCORSSafelistedRequestHeader(name, value, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEvent(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc, bool *_retval) override { return _to SendMouseEvent(aType, aX, aY, aButton, aClickCount, aModifiers, aIgnoreRootScrollFrame, aPressure, aInputSourceArg, aIsDOMEventSynthesized, aIsWidgetEventSynthesized, aButtons, aIdentifier, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEvent(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override { return _to SendTouchEvent(aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEventToWindow(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc) override { return _to SendMouseEventToWindow(aType, aX, aY, aButton, aClickCount, aModifiers, aIgnoreRootScrollFrame, aPressure, aInputSourceArg, aIsDOMEventSynthesized, aIsWidgetEventSynthesized, aButtons, aIdentifier, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEventToWindow(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override { return _to SendTouchEventToWindow(aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval); } \
  NS_IMETHOD SendWheelEvent(float aX, float aY, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aDeltaMode, int32_t aModifiers, int32_t aLineOrPageDeltaX, int32_t aLineOrPageDeltaY, uint32_t aOptions) override { return _to SendWheelEvent(aX, aY, aDeltaX, aDeltaY, aDeltaZ, aDeltaMode, aModifiers, aLineOrPageDeltaX, aLineOrPageDeltaY, aOptions); } \
  NS_IMETHOD SendNativeKeyEvent(int32_t aNativeKeyboardLayout, int32_t aNativeKeyCode, int32_t aModifierFlags, const nsAString& aCharacters, const nsAString& aUnmodifiedCharacters, nsIObserver *aObserver) override { return _to SendNativeKeyEvent(aNativeKeyboardLayout, aNativeKeyCode, aModifierFlags, aCharacters, aUnmodifiedCharacters, aObserver); } \
  NS_IMETHOD SendNativeMouseEvent(int32_t aScreenX, int32_t aScreenY, int32_t aNativeMessage, int32_t aModifierFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return _to SendNativeMouseEvent(aScreenX, aScreenY, aNativeMessage, aModifierFlags, aElement, aObserver); } \
  NS_IMETHOD SendNativeMouseMove(int32_t aScreenX, int32_t aScreenY, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return _to SendNativeMouseMove(aScreenX, aScreenY, aElement, aObserver); } \
  NS_IMETHOD SuppressAnimation(bool aSuppress) override { return _to SuppressAnimation(aSuppress); } \
  NS_IMETHOD SendNativeMouseScrollEvent(int32_t aScreenX, int32_t aScreenY, uint32_t aNativeMessage, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aModifierFlags, uint32_t aAdditionalFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return _to SendNativeMouseScrollEvent(aScreenX, aScreenY, aNativeMessage, aDeltaX, aDeltaY, aDeltaZ, aModifierFlags, aAdditionalFlags, aElement, aObserver); } \
  NS_IMETHOD SendNativeTouchPoint(uint32_t aPointerId, uint32_t aTouchState, int32_t aScreenX, int32_t aScreenY, double aPressure, uint32_t aOrientation, nsIObserver *aObserver) override { return _to SendNativeTouchPoint(aPointerId, aTouchState, aScreenX, aScreenY, aPressure, aOrientation, aObserver); } \
  NS_IMETHOD SendNativeTouchpadPinch(uint32_t aEventPhase, float aScale, int32_t aScreenX, int32_t aScreenY, int32_t aModifierFlags) override { return _to SendNativeTouchpadPinch(aEventPhase, aScale, aScreenX, aScreenY, aModifierFlags); } \
  NS_IMETHOD SendNativeTouchTap(int32_t aScreenX, int32_t aScreenY, bool aLongTap, nsIObserver *aObserver) override { return _to SendNativeTouchTap(aScreenX, aScreenY, aLongTap, aObserver); } \
  NS_IMETHOD ClearNativeTouchSequence(nsIObserver *aObserver) override { return _to ClearNativeTouchSequence(aObserver); } \
  NS_IMETHOD ClearSharedStyleSheetCache(void) override { return _to ClearSharedStyleSheetCache(); } \
  NS_IMETHOD GetParsedStyleSheets(uint32_t *aParsedStyleSheets) override { return _to GetParsedStyleSheets(aParsedStyleSheets); } \
  NS_IMETHOD ActivateNativeMenuItemAt(const nsAString& indexString) override { return _to ActivateNativeMenuItemAt(indexString); } \
  NS_IMETHOD ForceUpdateNativeMenuAt(const nsAString& indexString) override { return _to ForceUpdateNativeMenuAt(indexString); } \
  NS_IMETHOD GetSelectionAsPlaintext(nsAString& _retval) override { return _to GetSelectionAsPlaintext(_retval); } \
  NS_IMETHOD GarbageCollect(nsICycleCollectorListener *aListener) override { return _to GarbageCollect(aListener); } \
  NS_IMETHOD CycleCollect(nsICycleCollectorListener *aListener) override { return _to CycleCollect(aListener); } \
  NS_IMETHOD RunNextCollectorTimer(void) override { return _to RunNextCollectorTimer(); } \
  NS_IMETHOD SendSimpleGestureEvent(const nsAString& aType, float aX, float aY, uint32_t aDirection, double aDelta, int32_t aModifiers, uint32_t aClickCount) override { return _to SendSimpleGestureEvent(aType, aX, aY, aDirection, aDelta, aModifiers, aClickCount); } \
  NS_IMETHOD ElementFromPoint(float aX, float aY, bool aIgnoreRootScrollFrame, bool aFlushLayout, mozilla::dom::Element **_retval) override { return _to ElementFromPoint(aX, aY, aIgnoreRootScrollFrame, aFlushLayout, _retval); } \
  NS_IMETHOD NodesFromRect(float aX, float aY, float aTopSize, float aRightSize, float aBottomSize, float aLeftSize, bool aIgnoreRootScrollFrame, bool aFlushLayout, bool aOnlyVisible, float aTransparencyThreshold, nsINodeList **_retval) override { return _to NodesFromRect(aX, aY, aTopSize, aRightSize, aBottomSize, aLeftSize, aIgnoreRootScrollFrame, aFlushLayout, aOnlyVisible, aTransparencyThreshold, _retval); } \
  NS_IMETHOD GetTranslationNodes(nsINode *aRoot, nsITranslationNodeList **_retval) override { return _to GetTranslationNodes(aRoot, _retval); } \
  NS_IMETHOD CompareCanvases(nsISupports *aCanvas1, nsISupports *aCanvas2, uint32_t *aMaxDifference, uint32_t *_retval) override { return _to CompareCanvases(aCanvas1, aCanvas2, aMaxDifference, _retval); } \
  NS_IMETHOD GetIsMozAfterPaintPending(bool *aIsMozAfterPaintPending) override { return _to GetIsMozAfterPaintPending(aIsMozAfterPaintPending); } \
  NS_IMETHOD GetIsInputTaskManagerSuspended(bool *aIsInputTaskManagerSuspended) override { return _to GetIsInputTaskManagerSuspended(aIsInputTaskManagerSuspended); } \
  NS_IMETHOD SuppressEventHandling(bool aSuppress) override { return _to SuppressEventHandling(aSuppress); } \
  NS_IMETHOD DisableNonTestMouseEvents(bool aDisable) override { return _to DisableNonTestMouseEvents(aDisable); } \
  NS_IMETHOD GetScrollXY(bool aFlushLayout, int32_t *aScrollX, int32_t *aScrollY) override { return _to GetScrollXY(aFlushLayout, aScrollX, aScrollY); } \
  NS_IMETHOD GetScrollXYFloat(bool aFlushLayout, float *aScrollX, float *aScrollY) override { return _to GetScrollXYFloat(aFlushLayout, aScrollX, aScrollY); } \
  NS_IMETHOD GetScrollbarSize(bool aFlushLayout, int32_t *aWidth, int32_t *aHeight) override { return _to GetScrollbarSize(aFlushLayout, aWidth, aHeight); } \
  NS_IMETHOD GetBoundsWithoutFlushing(mozilla::dom::Element *aElement, mozilla::dom::DOMRect **_retval) override { return _to GetBoundsWithoutFlushing(aElement, _retval); } \
  NS_IMETHOD ScrollToVisual(float aOffsetX, float aOffsetY, int32_t aUpdateType, int32_t aScrollMode) override { return _to ScrollToVisual(aOffsetX, aOffsetY, aUpdateType, aScrollMode); } \
  NS_IMETHOD GetVisualViewportOffsetRelativeToLayoutViewport(float *aOffsetX, float *aOffsetY) override { return _to GetVisualViewportOffsetRelativeToLayoutViewport(aOffsetX, aOffsetY); } \
  NS_IMETHOD GetVisualViewportOffset(int32_t *aOffsetX, int32_t *aOffsetY) override { return _to GetVisualViewportOffset(aOffsetX, aOffsetY); } \
  NS_IMETHOD TransformRectLayoutToVisual(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override { return _to TransformRectLayoutToVisual(aX, aY, aWidth, aHeight, _retval); } \
  NS_IMETHOD ToScreenRect(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override { return _to ToScreenRect(aX, aY, aWidth, aHeight, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDynamicToolbarMaxHeight(uint32_t aHeightInScreen) override { return _to SetDynamicToolbarMaxHeight(aHeightInScreen); } \
  NS_IMETHOD NeedsFlush(int32_t aFlushtype, bool *_retval) override { return _to NeedsFlush(aFlushtype, _retval); } \
  NS_IMETHOD FlushLayoutWithoutThrottledAnimations(void) override { return _to FlushLayoutWithoutThrottledAnimations(); } \
  NS_IMETHOD GetRootBounds(mozilla::dom::DOMRect **_retval) override { return _to GetRootBounds(_retval); } \
  NS_IMETHOD GetIMEIsOpen(bool *aIMEIsOpen) override { return _to GetIMEIsOpen(aIMEIsOpen); } \
  NS_IMETHOD GetIMEStatus(uint32_t *aIMEStatus) override { return _to GetIMEStatus(aIMEStatus); } \
  NS_IMETHOD GetScreenPixelsPerCSSPixel(float *aScreenPixelsPerCSSPixel) override { return _to GetScreenPixelsPerCSSPixel(aScreenPixelsPerCSSPixel); } \
  NS_IMETHOD GetScreenPixelsPerCSSPixelNoOverride(float *aScreenPixelsPerCSSPixelNoOverride) override { return _to GetScreenPixelsPerCSSPixelNoOverride(aScreenPixelsPerCSSPixelNoOverride); } \
  NS_IMETHOD GetFullZoom(float *aFullZoom) override { return _to GetFullZoom(aFullZoom); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DispatchDOMEventViaPresShellForTesting(nsINode *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override { return _to DispatchDOMEventViaPresShellForTesting(aTarget, aEvent, _retval); } \
  NS_IMETHOD DispatchEventToChromeOnly(mozilla::dom::EventTarget *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override { return _to DispatchEventToChromeOnly(aTarget, aEvent, _retval); } \
  NS_IMETHOD GetClassName(JS::HandleValue aObject, JSContext* cx, char * *_retval) override { return _to GetClassName(aObject, cx, _retval); } \
  NS_IMETHOD SendContentCommandEvent(const nsAString& aType, nsITransferable *aTransferable) override { return _to SendContentCommandEvent(aType, aTransferable); } \
  NS_IMETHOD SendQueryContentEvent(uint32_t aType, int64_t aOffset, uint32_t aLength, int32_t aX, int32_t aY, uint32_t aAdditionalFlags, nsIQueryContentEventResult **_retval) override { return _to SendQueryContentEvent(aType, aOffset, aLength, aX, aY, aAdditionalFlags, _retval); } \
  NS_IMETHOD RemoteFrameFullscreenChanged(mozilla::dom::Element *aFrameElement) override { return _to RemoteFrameFullscreenChanged(aFrameElement); } \
  NS_IMETHOD RemoteFrameFullscreenReverted(void) override { return _to RemoteFrameFullscreenReverted(); } \
  NS_IMETHOD HandleFullscreenRequests(bool *_retval) override { return _to HandleFullscreenRequests(_retval); } \
  NS_IMETHOD ExitFullscreen(void) override { return _to ExitFullscreen(); } \
  NS_IMETHOD SendSelectionSetEvent(uint32_t aOffset, uint32_t aLength, uint32_t aAdditionalFlags, bool *_retval) override { return _to SendSelectionSetEvent(aOffset, aLength, aAdditionalFlags, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAtPoint(float aX, float aY, uint32_t aSelectBehavior, bool *_retval) override { return _to SelectAtPoint(aX, aY, aSelectBehavior, _retval); } \
  NS_IMETHOD GetVisitedDependentComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aPropertyName, nsAString& _retval) override { return _to GetVisitedDependentComputedStyle(aElement, aPseudoElement, aPropertyName, _retval); } \
  NS_IMETHOD GetDeprecatedOuterWindowID(uint64_t *aDeprecatedOuterWindowID) override { return _to GetDeprecatedOuterWindowID(aDeprecatedOuterWindowID); } \
  NS_IMETHOD EnterModalState(void) override { return _to EnterModalState(); } \
  NS_IMETHOD LeaveModalState(void) override { return _to LeaveModalState(); } \
  NS_IMETHOD IsInModalState(bool *_retval) override { return _to IsInModalState(_retval); } \
  NS_IMETHOD GetDesktopModeViewport(bool *aDesktopModeViewport) override { return _to GetDesktopModeViewport(aDesktopModeViewport); } \
  NS_IMETHOD SetDesktopModeViewport(bool aDesktopModeViewport) override { return _to SetDesktopModeViewport(aDesktopModeViewport); } \
  NS_IMETHOD SuspendTimeouts(void) override { return _to SuspendTimeouts(); } \
  NS_IMETHOD ResumeTimeouts(void) override { return _to ResumeTimeouts(); } \
  NS_IMETHOD GetLayerManagerType(nsAString& aLayerManagerType) override { return _to GetLayerManagerType(aLayerManagerType); } \
  NS_IMETHOD GetLayerManagerRemote(bool *aLayerManagerRemote) override { return _to GetLayerManagerRemote(aLayerManagerRemote); } \
  NS_IMETHOD GetUsingAdvancedLayers(bool *aUsingAdvancedLayers) override { return _to GetUsingAdvancedLayers(aUsingAdvancedLayers); } \
  NS_IMETHOD GetIsWebRenderRequested(bool *aIsWebRenderRequested) override { return _to GetIsWebRenderRequested(aIsWebRenderRequested); } \
  NS_IMETHOD GetCurrentAudioBackend(nsAString& aCurrentAudioBackend) override { return _to GetCurrentAudioBackend(aCurrentAudioBackend); } \
  NS_IMETHOD GetCurrentMaxAudioChannels(uint32_t *aCurrentMaxAudioChannels) override { return _to GetCurrentMaxAudioChannels(aCurrentMaxAudioChannels); } \
  NS_IMETHOD DefaultDevicesRoundTripLatency(::mozilla::dom::Promise * * _retval) override { return _to DefaultDevicesRoundTripLatency(_retval); } \
  NS_IMETHOD GetCurrentPreferredSampleRate(uint32_t *aCurrentPreferredSampleRate) override { return _to GetCurrentPreferredSampleRate(aCurrentPreferredSampleRate); } \
  NS_IMETHOD AudioDevices(uint16_t aSide, nsIArray **_retval) override { return _to AudioDevices(aSide, _retval); } \
  NS_IMETHOD StartFrameTimeRecording(uint32_t *startIndex) override { return _to StartFrameTimeRecording(startIndex); } \
  NS_IMETHOD StopFrameTimeRecording(uint32_t startIndex, nsTArray<float >& _retval) override { return _to StopFrameTimeRecording(startIndex, _retval); } \
  NS_IMETHOD GetDisplayDPI(float *aDisplayDPI) override { return _to GetDisplayDPI(aDisplayDPI); } \
  NS_IMETHOD AdvanceTimeAndRefresh(int64_t aMilliseconds) override { return _to AdvanceTimeAndRefresh(aMilliseconds); } \
  NS_IMETHOD RestoreNormalRefresh(void) override { return _to RestoreNormalRefresh(); } \
  NS_IMETHOD GetIsTestControllingRefreshes(bool *aIsTestControllingRefreshes) override { return _to GetIsTestControllingRefreshes(aIsTestControllingRefreshes); } \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override { return _to GetAsyncPanZoomEnabled(aAsyncPanZoomEnabled); } \
  NS_IMETHOD SetAsyncScrollOffset(mozilla::dom::Element *aElement, float aX, float aY) override { return _to SetAsyncScrollOffset(aElement, aX, aY); } \
  NS_IMETHOD SetAsyncZoom(mozilla::dom::Element *aRootElement, float aValue) override { return _to SetAsyncZoom(aRootElement, aValue); } \
  NS_IMETHOD FlushApzRepaints(bool *_retval) override { return _to FlushApzRepaints(_retval); } \
  NS_IMETHOD DisableApzForElement(mozilla::dom::Element *aElement) override { return _to DisableApzForElement(aElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ZoomToFocusedInput(void) override { return _to ZoomToFocusedInput(); } \
  NS_IMETHOD ComputeAnimationDistance(mozilla::dom::Element *element, const nsAString& property, const nsAString& value1, const nsAString& value2, double *_retval) override { return _to ComputeAnimationDistance(element, property, value1, value2, _retval); } \
  NS_IMETHOD GetUnanimatedComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aProperty, int32_t aFlushType, nsAString& _retval) override { return _to GetUnanimatedComputedStyle(aElement, aPseudoElement, aProperty, aFlushType, _retval); } \
  NS_IMETHOD GetFocusedInputType(nsAString& aFocusedInputType) override { return _to GetFocusedInputType(aFocusedInputType); } \
  NS_IMETHOD GetFocusedActionHint(nsAString& aFocusedActionHint) override { return _to GetFocusedActionHint(aFocusedActionHint); } \
  NS_IMETHOD GetFocusedInputMode(nsAString& aFocusedInputMode) override { return _to GetFocusedInputMode(aFocusedInputMode); } \
  NS_IMETHOD GetFocusedAutocapitalize(nsAString& aFocusedAutocapitalize) override { return _to GetFocusedAutocapitalize(aFocusedAutocapitalize); } \
  NS_IMETHOD GetViewId(mozilla::dom::Element *aElement, nsViewID *_retval) override { return _to GetViewId(aElement, _retval); } \
  NS_IMETHOD LeafLayersPartitionWindow(bool *_retval) override { return _to LeafLayersPartitionWindow(_retval); } \
  NS_IMETHOD CheckAndClearPaintedState(mozilla::dom::Element *aElement, bool *_retval) override { return _to CheckAndClearPaintedState(aElement, _retval); } \
  NS_IMETHOD CheckAndClearDisplayListState(mozilla::dom::Element *aElement, bool *_retval) override { return _to CheckAndClearDisplayListState(aElement, _retval); } \
  NS_IMETHOD IsPartOfOpaqueLayer(mozilla::dom::Element *aElement, bool *_retval) override { return _to IsPartOfOpaqueLayer(aElement, _retval); } \
  NS_IMETHOD NumberOfAssignedPaintedLayers(const nsTArray<RefPtr<mozilla::dom::Element>>& aElements, uint32_t *_retval) override { return _to NumberOfAssignedPaintedLayers(aElements, _retval); } \
  NS_IMETHOD GetFileId(JS::HandleValue aFile, JSContext* cx, int64_t *_retval) override { return _to GetFileId(aFile, cx, _retval); } \
  NS_IMETHOD GetFilePath(JS::HandleValue aFile, JSContext* cx, nsAString& _retval) override { return _to GetFilePath(aFile, cx, _retval); } \
  NS_IMETHOD GetFileReferences(const nsAString& aDatabaseName, int64_t aId, JS::HandleValue aOptions, int32_t *aRefCnt, int32_t *aDBRefCnt, JSContext* cx, bool *_retval) override { return _to GetFileReferences(aDatabaseName, aId, aOptions, aRefCnt, aDBRefCnt, cx, _retval); } \
  NS_IMETHOD FlushPendingFileDeletions(void) override { return _to FlushPendingFileDeletions(); } \
  NS_IMETHOD StartPCCountProfiling(JSContext* cx) override { return _to StartPCCountProfiling(cx); } \
  NS_IMETHOD StopPCCountProfiling(JSContext* cx) override { return _to StopPCCountProfiling(cx); } \
  NS_IMETHOD PurgePCCounts(JSContext* cx) override { return _to PurgePCCounts(cx); } \
  NS_IMETHOD GetPCCountScriptCount(JSContext* cx, int32_t *_retval) override { return _to GetPCCountScriptCount(cx, _retval); } \
  NS_IMETHOD GetPCCountScriptSummary(int32_t script, JSContext* cx, nsAString& _retval) override { return _to GetPCCountScriptSummary(script, cx, _retval); } \
  NS_IMETHOD GetPCCountScriptContents(int32_t script, JSContext* cx, nsAString& _retval) override { return _to GetPCCountScriptContents(script, cx, _retval); } \
  NS_IMETHOD GetPaintingSuppressed(bool *aPaintingSuppressed) override { return _to GetPaintingSuppressed(aPaintingSuppressed); } \
  NS_IMETHOD GetPlugins(JSContext* cx, JS::MutableHandleValue aPlugins) override { return _to GetPlugins(cx, aPlugins); } \
  NS_IMETHOD SetVisualViewportSize(float aWidth, float aHeight) override { return _to SetVisualViewportSize(aWidth, aHeight); } \
  NS_IMETHOD DisableDialogs(void) override { return _to DisableDialogs(); } \
  NS_IMETHOD EnableDialogs(void) override { return _to EnableDialogs(); } \
  NS_IMETHOD AreDialogsEnabled(bool *_retval) override { return _to AreDialogsEnabled(_retval); } \
  NS_IMETHOD LoadSheet(nsIURI *sheetURI, uint32_t type) override { return _to LoadSheet(sheetURI, type); } \
  NS_IMETHOD LoadSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override { return _to LoadSheetUsingURIString(sheetURI, type); } \
  NS_IMETHOD AddSheet(nsIPreloadedStyleSheet *sheet, uint32_t type) override { return _to AddSheet(sheet, type); } \
  NS_IMETHOD RemoveSheet(nsIURI *sheetURI, uint32_t type) override { return _to RemoveSheet(sheetURI, type); } \
  NS_IMETHOD RemoveSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override { return _to RemoveSheetUsingURIString(sheetURI, type); } \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override { return _to GetIsHandlingUserInput(aIsHandlingUserInput); } \
  NS_IMETHOD GetMillisSinceLastUserInput(double *aMillisSinceLastUserInput) override { return _to GetMillisSinceLastUserInput(aMillisSinceLastUserInput); } \
  NS_IMETHOD AllowScriptsToClose(void) override { return _to AllowScriptsToClose(); } \
  NS_IMETHOD GetIsParentWindowMainWidgetVisible(bool *aIsParentWindowMainWidgetVisible) override { return _to GetIsParentWindowMainWidgetVisible(aIsParentWindowMainWidgetVisible); } \
  NS_IMETHOD IsNodeDisabledForEvents(nsINode *aNode, bool *_retval) override { return _to IsNodeDisabledForEvents(aNode, _retval); } \
  NS_IMETHOD GetPaintFlashing(bool *aPaintFlashing) override { return _to GetPaintFlashing(aPaintFlashing); } \
  NS_IMETHOD SetPaintFlashing(bool aPaintFlashing) override { return _to SetPaintFlashing(aPaintFlashing); } \
  NS_IMETHOD GetOMTAStyle(mozilla::dom::Element *aElement, const nsAString& aProperty, const nsAString& aPseudoElement, nsAString& _retval) override { return _to GetOMTAStyle(aElement, aProperty, aPseudoElement, _retval); } \
  NS_IMETHOD GetOMTCTransform(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, nsAString& _retval) override { return _to GetOMTCTransform(aElement, aPseudoElement, _retval); } \
  NS_IMETHOD IsAnimationInPendingTracker(mozilla::dom::Animation *aAnimation, bool *_retval) override { return _to IsAnimationInPendingTracker(aAnimation, _retval); } \
  NS_IMETHOD SetHandlingUserInput(bool aHandlingInput, nsIJSRAIIHelper **_retval) override { return _to SetHandlingUserInput(aHandlingInput, _retval); } \
  NS_IMETHOD GetContentAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetContentAPZTestData(cx, _retval); } \
  NS_IMETHOD GetCompositorAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetCompositorAPZTestData(cx, _retval); } \
  NS_IMETHOD PostRestyleSelfEvent(mozilla::dom::Element *aElement) override { return _to PostRestyleSelfEvent(aElement); } \
  NS_IMETHOD XpconnectArgument(nsISupports *aObj) override { return _to XpconnectArgument(aObj); } \
  NS_IMETHOD AskPermission(nsIContentPermissionRequest *aRequest) override { return _to AskPermission(aRequest); } \
  NS_IMETHOD GetRestyleGeneration(uint64_t *aRestyleGeneration) override { return _to GetRestyleGeneration(aRestyleGeneration); } \
  NS_IMETHOD GetFramesConstructed(uint64_t *aFramesConstructed) override { return _to GetFramesConstructed(aFramesConstructed); } \
  NS_IMETHOD GetFramesReflowed(uint64_t *aFramesReflowed) override { return _to GetFramesReflowed(aFramesReflowed); } \
  NS_IMETHOD SetChromeMargin(int32_t aTop, int32_t aRight, int32_t aBottom, int32_t aLeft) override { return _to SetChromeMargin(aTop, aRight, aBottom, aLeft); } \
  NS_IMETHOD GetFrameUniformityTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return _to GetFrameUniformityTestData(cx, _retval); } \
  NS_IMETHOD EnterChaosMode(void) override { return _to EnterChaosMode(); } \
  NS_IMETHOD LeaveChaosMode(void) override { return _to LeaveChaosMode(); } \
  NS_IMETHOD TriggerDeviceReset(void) override { return _to TriggerDeviceReset(); } \
  NS_IMETHOD HasRuleProcessorUsedByMultipleStyleSets(uint32_t aSheetType, bool *_retval) override { return _to HasRuleProcessorUsedByMultipleStyleSets(aSheetType, _retval); } \
  NS_IMETHOD RespectDisplayPortSuppression(bool aEnabled) override { return _to RespectDisplayPortSuppression(aEnabled); } \
  NS_IMETHOD ForceReflowInterrupt(void) override { return _to ForceReflowInterrupt(); } \
  NS_IMETHOD TerminateGPUProcess(void) override { return _to TerminateGPUProcess(); } \
  NS_IMETHOD GetGpuProcessPid(int32_t *aGpuProcessPid) override { return _to GetGpuProcessPid(aGpuProcessPid); } \
  NS_IMETHOD AddManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override { return _to AddManuallyManagedState(element, state); } \
  NS_IMETHOD RemoveManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override { return _to RemoveManuallyManagedState(element, state); } \
  NS_IMETHOD GetStorageUsage(mozilla::dom::Storage *aStorage, int64_t *_retval) override { return _to GetStorageUsage(aStorage, _retval); } \
  NS_IMETHOD GetDirectionFromText(const nsAString& aString, int32_t *_retval) override { return _to GetDirectionFromText(aString, _retval); } \
  NS_IMETHOD EnsureDirtyRootFrame(void) override { return _to EnsureDirtyRootFrame(); } \
  NS_IMETHOD WrCapture(void) override { return _to WrCapture(); } \
  NS_IMETHOD WrToggleCaptureSequence(void) override { return _to WrToggleCaptureSequence(); } \
  NS_IMETHOD SetCompositionRecording(bool aValue, ::mozilla::dom::Promise * * _retval) override { return _to SetCompositionRecording(aValue, _retval); } \
  NS_IMETHOD StartCompositionRecording(::mozilla::dom::Promise * * _retval) override { return _to StartCompositionRecording(_retval); } \
  NS_IMETHOD StopCompositionRecording(bool aWriteToDisk, ::mozilla::dom::Promise * * _retval) override { return _to StopCompositionRecording(aWriteToDisk, _retval); } \
  NS_IMETHOD IsCssPropertyRecordedInUseCounter(const nsACString& aProperty, bool *_retval) override { return _to IsCssPropertyRecordedInUseCounter(aProperty, _retval); } \
  NS_IMETHOD ResetMobileViewportManager(void) override { return _to ResetMobileViewportManager(); } \
  NS_IMETHOD GetSystemFont(nsACString& aSystemFont) override { return _to GetSystemFont(aSystemFont); } \
  NS_IMETHOD SetSystemFont(const nsACString& aSystemFont) override { return _to SetSystemFont(aSystemFont); } \
  NS_IMETHOD GetPaintCount(uint64_t *aPaintCount) override { return _to GetPaintCount(aPaintCount); } \
  NS_IMETHOD SyncFlushCompositor(void) override { return _to SyncFlushCompositor(); } \
  NS_IMETHOD GetLayersId(uint64_t *_retval) override { return _to GetLayersId(_retval); } \
  NS_IMETHOD GetUsesOverlayScrollbars(bool *aUsesOverlayScrollbars) override { return _to GetUsesOverlayScrollbars(aUsesOverlayScrollbars); } \
  NS_IMETHOD GetEffectivelyThrottlesFrameRequests(bool *aEffectivelyThrottlesFrameRequests) override { return _to GetEffectivelyThrottlesFrameRequests(aEffectivelyThrottlesFrameRequests); } \
  NS_IMETHOD GetWebrtcRawDeviceId(nsAString& aWebrtcRawDeviceId) override { return _to GetWebrtcRawDeviceId(aWebrtcRawDeviceId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMWINDOWUTILS(_to) \
  NS_IMETHOD GetImageAnimationMode(uint16_t *aImageAnimationMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageAnimationMode(aImageAnimationMode); } \
  NS_IMETHOD SetImageAnimationMode(uint16_t aImageAnimationMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetImageAnimationMode(aImageAnimationMode); } \
  NS_IMETHOD GetDocCharsetIsForced(bool *aDocCharsetIsForced) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocCharsetIsForced(aDocCharsetIsForced); } \
  NS_IMETHOD GetPhysicalMillimeterInCSSPixels(float *aPhysicalMillimeterInCSSPixels) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPhysicalMillimeterInCSSPixels(aPhysicalMillimeterInCSSPixels); } \
  NS_IMETHOD GetDocumentMetadata(const nsAString& aName, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentMetadata(aName, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateLayerTree(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateLayerTree(); } \
  NS_IMETHOD GetLastTransactionId(uint64_t *aLastTransactionId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastTransactionId(aLastTransactionId); } \
  NS_IMETHOD GetViewportInfo(uint32_t aDisplayWidth, uint32_t aDisplayHeight, double *aDefaultZoom, bool *aAllowZoom, double *aMinZoom, double *aMaxZoom, uint32_t *aWidth, uint32_t *aHeight, bool *aAutoSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetViewportInfo(aDisplayWidth, aDisplayHeight, aDefaultZoom, aAllowZoom, aMinZoom, aMaxZoom, aWidth, aHeight, aAutoSize); } \
  NS_IMETHOD GetViewportFitInfo(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetViewportFitInfo(_retval); } \
  NS_IMETHOD GetContentViewerSize(uint32_t *aDisplayWidth, uint32_t *aDisplayHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentViewerSize(aDisplayWidth, aDisplayHeight); } \
  NS_IMETHOD SetDisplayPortForElement(float aXPx, float aYPx, float aWidthPx, float aHeightPx, mozilla::dom::Element *aElement, uint32_t aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplayPortForElement(aXPx, aYPx, aWidthPx, aHeightPx, aElement, aPriority); } \
  NS_IMETHOD SetDisplayPortMarginsForElement(float aLeftMargin, float aTopMargin, float aRightMargin, float aBottomMargin, mozilla::dom::Element *aElement, uint32_t aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplayPortMarginsForElement(aLeftMargin, aTopMargin, aRightMargin, aBottomMargin, aElement, aPriority); } \
  NS_IMETHOD SetDisplayPortBaseForElement(int32_t aX, int32_t aY, int32_t aWidth, int32_t aHeight, mozilla::dom::Element *aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplayPortBaseForElement(aX, aY, aWidth, aHeight, aElement); } \
  NS_IMETHOD GetScrollbarSizes(mozilla::dom::Element *aElement, uint32_t *aVerticalScrollbarWidth, uint32_t *aHorizontalScrollbarHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollbarSizes(aElement, aVerticalScrollbarWidth, aHorizontalScrollbarHeight); } \
  NS_IMETHOD SetResolutionAndScaleTo(float aResolution) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResolutionAndScaleTo(aResolution); } \
  NS_IMETHOD GetResolution(float *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResolution(_retval); } \
  NS_IMETHOD SetRestoreResolution(float aResolution, uint32_t aDisplayWidth, uint32_t aDisplayHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRestoreResolution(aResolution, aDisplayWidth, aDisplayHeight); } \
  NS_IMETHOD GetIsFirstPaint(bool *aIsFirstPaint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFirstPaint(aIsFirstPaint); } \
  NS_IMETHOD SetIsFirstPaint(bool aIsFirstPaint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsFirstPaint(aIsFirstPaint); } \
  NS_IMETHOD GetPresShellId(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPresShellId(_retval); } \
  NS_IMETHOD IsCORSSafelistedRequestHeader(const nsACString& name, const nsACString& value, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCORSSafelistedRequestHeader(name, value, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEvent(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMouseEvent(aType, aX, aY, aButton, aClickCount, aModifiers, aIgnoreRootScrollFrame, aPressure, aInputSourceArg, aIsDOMEventSynthesized, aIsWidgetEventSynthesized, aButtons, aIdentifier, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEvent(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendTouchEvent(aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendMouseEventToWindow(const nsAString& aType, float aX, float aY, int32_t aButton, int32_t aClickCount, int32_t aModifiers, bool aIgnoreRootScrollFrame, float aPressure, uint16_t aInputSourceArg, bool aIsDOMEventSynthesized, bool aIsWidgetEventSynthesized, int32_t aButtons, uint32_t aIdentifier, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMouseEventToWindow(aType, aX, aY, aButton, aClickCount, aModifiers, aIgnoreRootScrollFrame, aPressure, aInputSourceArg, aIsDOMEventSynthesized, aIsWidgetEventSynthesized, aButtons, aIdentifier, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SendTouchEventToWindow(const nsAString& aType, const nsTArray<uint32_t >& aIdentifiers, const nsTArray<int32_t >& aXs, const nsTArray<int32_t >& aYs, const nsTArray<uint32_t >& aRxs, const nsTArray<uint32_t >& aRys, const nsTArray<float >& aRotationAngles, const nsTArray<float >& aForces, int32_t aModifiers, bool aIgnoreRootScrollFrame, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendTouchEventToWindow(aType, aIdentifiers, aXs, aYs, aRxs, aRys, aRotationAngles, aForces, aModifiers, aIgnoreRootScrollFrame, _retval); } \
  NS_IMETHOD SendWheelEvent(float aX, float aY, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aDeltaMode, int32_t aModifiers, int32_t aLineOrPageDeltaX, int32_t aLineOrPageDeltaY, uint32_t aOptions) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendWheelEvent(aX, aY, aDeltaX, aDeltaY, aDeltaZ, aDeltaMode, aModifiers, aLineOrPageDeltaX, aLineOrPageDeltaY, aOptions); } \
  NS_IMETHOD SendNativeKeyEvent(int32_t aNativeKeyboardLayout, int32_t aNativeKeyCode, int32_t aModifierFlags, const nsAString& aCharacters, const nsAString& aUnmodifiedCharacters, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeKeyEvent(aNativeKeyboardLayout, aNativeKeyCode, aModifierFlags, aCharacters, aUnmodifiedCharacters, aObserver); } \
  NS_IMETHOD SendNativeMouseEvent(int32_t aScreenX, int32_t aScreenY, int32_t aNativeMessage, int32_t aModifierFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeMouseEvent(aScreenX, aScreenY, aNativeMessage, aModifierFlags, aElement, aObserver); } \
  NS_IMETHOD SendNativeMouseMove(int32_t aScreenX, int32_t aScreenY, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeMouseMove(aScreenX, aScreenY, aElement, aObserver); } \
  NS_IMETHOD SuppressAnimation(bool aSuppress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuppressAnimation(aSuppress); } \
  NS_IMETHOD SendNativeMouseScrollEvent(int32_t aScreenX, int32_t aScreenY, uint32_t aNativeMessage, double aDeltaX, double aDeltaY, double aDeltaZ, uint32_t aModifierFlags, uint32_t aAdditionalFlags, mozilla::dom::Element *aElement, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeMouseScrollEvent(aScreenX, aScreenY, aNativeMessage, aDeltaX, aDeltaY, aDeltaZ, aModifierFlags, aAdditionalFlags, aElement, aObserver); } \
  NS_IMETHOD SendNativeTouchPoint(uint32_t aPointerId, uint32_t aTouchState, int32_t aScreenX, int32_t aScreenY, double aPressure, uint32_t aOrientation, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeTouchPoint(aPointerId, aTouchState, aScreenX, aScreenY, aPressure, aOrientation, aObserver); } \
  NS_IMETHOD SendNativeTouchpadPinch(uint32_t aEventPhase, float aScale, int32_t aScreenX, int32_t aScreenY, int32_t aModifierFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeTouchpadPinch(aEventPhase, aScale, aScreenX, aScreenY, aModifierFlags); } \
  NS_IMETHOD SendNativeTouchTap(int32_t aScreenX, int32_t aScreenY, bool aLongTap, nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendNativeTouchTap(aScreenX, aScreenY, aLongTap, aObserver); } \
  NS_IMETHOD ClearNativeTouchSequence(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearNativeTouchSequence(aObserver); } \
  NS_IMETHOD ClearSharedStyleSheetCache(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearSharedStyleSheetCache(); } \
  NS_IMETHOD GetParsedStyleSheets(uint32_t *aParsedStyleSheets) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParsedStyleSheets(aParsedStyleSheets); } \
  NS_IMETHOD ActivateNativeMenuItemAt(const nsAString& indexString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ActivateNativeMenuItemAt(indexString); } \
  NS_IMETHOD ForceUpdateNativeMenuAt(const nsAString& indexString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceUpdateNativeMenuAt(indexString); } \
  NS_IMETHOD GetSelectionAsPlaintext(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionAsPlaintext(_retval); } \
  NS_IMETHOD GarbageCollect(nsICycleCollectorListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GarbageCollect(aListener); } \
  NS_IMETHOD CycleCollect(nsICycleCollectorListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CycleCollect(aListener); } \
  NS_IMETHOD RunNextCollectorTimer(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunNextCollectorTimer(); } \
  NS_IMETHOD SendSimpleGestureEvent(const nsAString& aType, float aX, float aY, uint32_t aDirection, double aDelta, int32_t aModifiers, uint32_t aClickCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendSimpleGestureEvent(aType, aX, aY, aDirection, aDelta, aModifiers, aClickCount); } \
  NS_IMETHOD ElementFromPoint(float aX, float aY, bool aIgnoreRootScrollFrame, bool aFlushLayout, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ElementFromPoint(aX, aY, aIgnoreRootScrollFrame, aFlushLayout, _retval); } \
  NS_IMETHOD NodesFromRect(float aX, float aY, float aTopSize, float aRightSize, float aBottomSize, float aLeftSize, bool aIgnoreRootScrollFrame, bool aFlushLayout, bool aOnlyVisible, float aTransparencyThreshold, nsINodeList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NodesFromRect(aX, aY, aTopSize, aRightSize, aBottomSize, aLeftSize, aIgnoreRootScrollFrame, aFlushLayout, aOnlyVisible, aTransparencyThreshold, _retval); } \
  NS_IMETHOD GetTranslationNodes(nsINode *aRoot, nsITranslationNodeList **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTranslationNodes(aRoot, _retval); } \
  NS_IMETHOD CompareCanvases(nsISupports *aCanvas1, nsISupports *aCanvas2, uint32_t *aMaxDifference, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompareCanvases(aCanvas1, aCanvas2, aMaxDifference, _retval); } \
  NS_IMETHOD GetIsMozAfterPaintPending(bool *aIsMozAfterPaintPending) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsMozAfterPaintPending(aIsMozAfterPaintPending); } \
  NS_IMETHOD GetIsInputTaskManagerSuspended(bool *aIsInputTaskManagerSuspended) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInputTaskManagerSuspended(aIsInputTaskManagerSuspended); } \
  NS_IMETHOD SuppressEventHandling(bool aSuppress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuppressEventHandling(aSuppress); } \
  NS_IMETHOD DisableNonTestMouseEvents(bool aDisable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisableNonTestMouseEvents(aDisable); } \
  NS_IMETHOD GetScrollXY(bool aFlushLayout, int32_t *aScrollX, int32_t *aScrollY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollXY(aFlushLayout, aScrollX, aScrollY); } \
  NS_IMETHOD GetScrollXYFloat(bool aFlushLayout, float *aScrollX, float *aScrollY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollXYFloat(aFlushLayout, aScrollX, aScrollY); } \
  NS_IMETHOD GetScrollbarSize(bool aFlushLayout, int32_t *aWidth, int32_t *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollbarSize(aFlushLayout, aWidth, aHeight); } \
  NS_IMETHOD GetBoundsWithoutFlushing(mozilla::dom::Element *aElement, mozilla::dom::DOMRect **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBoundsWithoutFlushing(aElement, _retval); } \
  NS_IMETHOD ScrollToVisual(float aOffsetX, float aOffsetY, int32_t aUpdateType, int32_t aScrollMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScrollToVisual(aOffsetX, aOffsetY, aUpdateType, aScrollMode); } \
  NS_IMETHOD GetVisualViewportOffsetRelativeToLayoutViewport(float *aOffsetX, float *aOffsetY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisualViewportOffsetRelativeToLayoutViewport(aOffsetX, aOffsetY); } \
  NS_IMETHOD GetVisualViewportOffset(int32_t *aOffsetX, int32_t *aOffsetY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisualViewportOffset(aOffsetX, aOffsetY); } \
  NS_IMETHOD TransformRectLayoutToVisual(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TransformRectLayoutToVisual(aX, aY, aWidth, aHeight, _retval); } \
  NS_IMETHOD ToScreenRect(float aX, float aY, float aWidth, float aHeight, mozilla::dom::DOMRect **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ToScreenRect(aX, aY, aWidth, aHeight, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetDynamicToolbarMaxHeight(uint32_t aHeightInScreen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDynamicToolbarMaxHeight(aHeightInScreen); } \
  NS_IMETHOD NeedsFlush(int32_t aFlushtype, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NeedsFlush(aFlushtype, _retval); } \
  NS_IMETHOD FlushLayoutWithoutThrottledAnimations(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlushLayoutWithoutThrottledAnimations(); } \
  NS_IMETHOD GetRootBounds(mozilla::dom::DOMRect **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootBounds(_retval); } \
  NS_IMETHOD GetIMEIsOpen(bool *aIMEIsOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIMEIsOpen(aIMEIsOpen); } \
  NS_IMETHOD GetIMEStatus(uint32_t *aIMEStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIMEStatus(aIMEStatus); } \
  NS_IMETHOD GetScreenPixelsPerCSSPixel(float *aScreenPixelsPerCSSPixel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScreenPixelsPerCSSPixel(aScreenPixelsPerCSSPixel); } \
  NS_IMETHOD GetScreenPixelsPerCSSPixelNoOverride(float *aScreenPixelsPerCSSPixelNoOverride) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScreenPixelsPerCSSPixelNoOverride(aScreenPixelsPerCSSPixelNoOverride); } \
  NS_IMETHOD GetFullZoom(float *aFullZoom) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFullZoom(aFullZoom); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD DispatchDOMEventViaPresShellForTesting(nsINode *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchDOMEventViaPresShellForTesting(aTarget, aEvent, _retval); } \
  NS_IMETHOD DispatchEventToChromeOnly(mozilla::dom::EventTarget *aTarget, mozilla::dom::Event *aEvent, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DispatchEventToChromeOnly(aTarget, aEvent, _retval); } \
  NS_IMETHOD GetClassName(JS::HandleValue aObject, JSContext* cx, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassName(aObject, cx, _retval); } \
  NS_IMETHOD SendContentCommandEvent(const nsAString& aType, nsITransferable *aTransferable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendContentCommandEvent(aType, aTransferable); } \
  NS_IMETHOD SendQueryContentEvent(uint32_t aType, int64_t aOffset, uint32_t aLength, int32_t aX, int32_t aY, uint32_t aAdditionalFlags, nsIQueryContentEventResult **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendQueryContentEvent(aType, aOffset, aLength, aX, aY, aAdditionalFlags, _retval); } \
  NS_IMETHOD RemoteFrameFullscreenChanged(mozilla::dom::Element *aFrameElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteFrameFullscreenChanged(aFrameElement); } \
  NS_IMETHOD RemoteFrameFullscreenReverted(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteFrameFullscreenReverted(); } \
  NS_IMETHOD HandleFullscreenRequests(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleFullscreenRequests(_retval); } \
  NS_IMETHOD ExitFullscreen(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitFullscreen(); } \
  NS_IMETHOD SendSelectionSetEvent(uint32_t aOffset, uint32_t aLength, uint32_t aAdditionalFlags, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendSelectionSetEvent(aOffset, aLength, aAdditionalFlags, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SelectAtPoint(float aX, float aY, uint32_t aSelectBehavior, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectAtPoint(aX, aY, aSelectBehavior, _retval); } \
  NS_IMETHOD GetVisitedDependentComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aPropertyName, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisitedDependentComputedStyle(aElement, aPseudoElement, aPropertyName, _retval); } \
  NS_IMETHOD GetDeprecatedOuterWindowID(uint64_t *aDeprecatedOuterWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeprecatedOuterWindowID(aDeprecatedOuterWindowID); } \
  NS_IMETHOD EnterModalState(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnterModalState(); } \
  NS_IMETHOD LeaveModalState(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LeaveModalState(); } \
  NS_IMETHOD IsInModalState(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsInModalState(_retval); } \
  NS_IMETHOD GetDesktopModeViewport(bool *aDesktopModeViewport) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDesktopModeViewport(aDesktopModeViewport); } \
  NS_IMETHOD SetDesktopModeViewport(bool aDesktopModeViewport) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDesktopModeViewport(aDesktopModeViewport); } \
  NS_IMETHOD SuspendTimeouts(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuspendTimeouts(); } \
  NS_IMETHOD ResumeTimeouts(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeTimeouts(); } \
  NS_IMETHOD GetLayerManagerType(nsAString& aLayerManagerType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLayerManagerType(aLayerManagerType); } \
  NS_IMETHOD GetLayerManagerRemote(bool *aLayerManagerRemote) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLayerManagerRemote(aLayerManagerRemote); } \
  NS_IMETHOD GetUsingAdvancedLayers(bool *aUsingAdvancedLayers) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsingAdvancedLayers(aUsingAdvancedLayers); } \
  NS_IMETHOD GetIsWebRenderRequested(bool *aIsWebRenderRequested) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsWebRenderRequested(aIsWebRenderRequested); } \
  NS_IMETHOD GetCurrentAudioBackend(nsAString& aCurrentAudioBackend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentAudioBackend(aCurrentAudioBackend); } \
  NS_IMETHOD GetCurrentMaxAudioChannels(uint32_t *aCurrentMaxAudioChannels) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentMaxAudioChannels(aCurrentMaxAudioChannels); } \
  NS_IMETHOD DefaultDevicesRoundTripLatency(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DefaultDevicesRoundTripLatency(_retval); } \
  NS_IMETHOD GetCurrentPreferredSampleRate(uint32_t *aCurrentPreferredSampleRate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentPreferredSampleRate(aCurrentPreferredSampleRate); } \
  NS_IMETHOD AudioDevices(uint16_t aSide, nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AudioDevices(aSide, _retval); } \
  NS_IMETHOD StartFrameTimeRecording(uint32_t *startIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartFrameTimeRecording(startIndex); } \
  NS_IMETHOD StopFrameTimeRecording(uint32_t startIndex, nsTArray<float >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopFrameTimeRecording(startIndex, _retval); } \
  NS_IMETHOD GetDisplayDPI(float *aDisplayDPI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayDPI(aDisplayDPI); } \
  NS_IMETHOD AdvanceTimeAndRefresh(int64_t aMilliseconds) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdvanceTimeAndRefresh(aMilliseconds); } \
  NS_IMETHOD RestoreNormalRefresh(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RestoreNormalRefresh(); } \
  NS_IMETHOD GetIsTestControllingRefreshes(bool *aIsTestControllingRefreshes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTestControllingRefreshes(aIsTestControllingRefreshes); } \
  NS_IMETHOD GetAsyncPanZoomEnabled(bool *aAsyncPanZoomEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsyncPanZoomEnabled(aAsyncPanZoomEnabled); } \
  NS_IMETHOD SetAsyncScrollOffset(mozilla::dom::Element *aElement, float aX, float aY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsyncScrollOffset(aElement, aX, aY); } \
  NS_IMETHOD SetAsyncZoom(mozilla::dom::Element *aRootElement, float aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAsyncZoom(aRootElement, aValue); } \
  NS_IMETHOD FlushApzRepaints(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlushApzRepaints(_retval); } \
  NS_IMETHOD DisableApzForElement(mozilla::dom::Element *aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisableApzForElement(aElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ZoomToFocusedInput(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ZoomToFocusedInput(); } \
  NS_IMETHOD ComputeAnimationDistance(mozilla::dom::Element *element, const nsAString& property, const nsAString& value1, const nsAString& value2, double *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ComputeAnimationDistance(element, property, value1, value2, _retval); } \
  NS_IMETHOD GetUnanimatedComputedStyle(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, const nsAString& aProperty, int32_t aFlushType, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnanimatedComputedStyle(aElement, aPseudoElement, aProperty, aFlushType, _retval); } \
  NS_IMETHOD GetFocusedInputType(nsAString& aFocusedInputType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedInputType(aFocusedInputType); } \
  NS_IMETHOD GetFocusedActionHint(nsAString& aFocusedActionHint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedActionHint(aFocusedActionHint); } \
  NS_IMETHOD GetFocusedInputMode(nsAString& aFocusedInputMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedInputMode(aFocusedInputMode); } \
  NS_IMETHOD GetFocusedAutocapitalize(nsAString& aFocusedAutocapitalize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedAutocapitalize(aFocusedAutocapitalize); } \
  NS_IMETHOD GetViewId(mozilla::dom::Element *aElement, nsViewID *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetViewId(aElement, _retval); } \
  NS_IMETHOD LeafLayersPartitionWindow(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LeafLayersPartitionWindow(_retval); } \
  NS_IMETHOD CheckAndClearPaintedState(mozilla::dom::Element *aElement, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckAndClearPaintedState(aElement, _retval); } \
  NS_IMETHOD CheckAndClearDisplayListState(mozilla::dom::Element *aElement, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckAndClearDisplayListState(aElement, _retval); } \
  NS_IMETHOD IsPartOfOpaqueLayer(mozilla::dom::Element *aElement, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPartOfOpaqueLayer(aElement, _retval); } \
  NS_IMETHOD NumberOfAssignedPaintedLayers(const nsTArray<RefPtr<mozilla::dom::Element>>& aElements, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NumberOfAssignedPaintedLayers(aElements, _retval); } \
  NS_IMETHOD GetFileId(JS::HandleValue aFile, JSContext* cx, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileId(aFile, cx, _retval); } \
  NS_IMETHOD GetFilePath(JS::HandleValue aFile, JSContext* cx, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilePath(aFile, cx, _retval); } \
  NS_IMETHOD GetFileReferences(const nsAString& aDatabaseName, int64_t aId, JS::HandleValue aOptions, int32_t *aRefCnt, int32_t *aDBRefCnt, JSContext* cx, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileReferences(aDatabaseName, aId, aOptions, aRefCnt, aDBRefCnt, cx, _retval); } \
  NS_IMETHOD FlushPendingFileDeletions(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlushPendingFileDeletions(); } \
  NS_IMETHOD StartPCCountProfiling(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartPCCountProfiling(cx); } \
  NS_IMETHOD StopPCCountProfiling(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopPCCountProfiling(cx); } \
  NS_IMETHOD PurgePCCounts(JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PurgePCCounts(cx); } \
  NS_IMETHOD GetPCCountScriptCount(JSContext* cx, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPCCountScriptCount(cx, _retval); } \
  NS_IMETHOD GetPCCountScriptSummary(int32_t script, JSContext* cx, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPCCountScriptSummary(script, cx, _retval); } \
  NS_IMETHOD GetPCCountScriptContents(int32_t script, JSContext* cx, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPCCountScriptContents(script, cx, _retval); } \
  NS_IMETHOD GetPaintingSuppressed(bool *aPaintingSuppressed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaintingSuppressed(aPaintingSuppressed); } \
  NS_IMETHOD GetPlugins(JSContext* cx, JS::MutableHandleValue aPlugins) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlugins(cx, aPlugins); } \
  NS_IMETHOD SetVisualViewportSize(float aWidth, float aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetVisualViewportSize(aWidth, aHeight); } \
  NS_IMETHOD DisableDialogs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DisableDialogs(); } \
  NS_IMETHOD EnableDialogs(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableDialogs(); } \
  NS_IMETHOD AreDialogsEnabled(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AreDialogsEnabled(_retval); } \
  NS_IMETHOD LoadSheet(nsIURI *sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadSheet(sheetURI, type); } \
  NS_IMETHOD LoadSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadSheetUsingURIString(sheetURI, type); } \
  NS_IMETHOD AddSheet(nsIPreloadedStyleSheet *sheet, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddSheet(sheet, type); } \
  NS_IMETHOD RemoveSheet(nsIURI *sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveSheet(sheetURI, type); } \
  NS_IMETHOD RemoveSheetUsingURIString(const nsACString& sheetURI, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveSheetUsingURIString(sheetURI, type); } \
  NS_IMETHOD GetIsHandlingUserInput(bool *aIsHandlingUserInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsHandlingUserInput(aIsHandlingUserInput); } \
  NS_IMETHOD GetMillisSinceLastUserInput(double *aMillisSinceLastUserInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMillisSinceLastUserInput(aMillisSinceLastUserInput); } \
  NS_IMETHOD AllowScriptsToClose(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowScriptsToClose(); } \
  NS_IMETHOD GetIsParentWindowMainWidgetVisible(bool *aIsParentWindowMainWidgetVisible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsParentWindowMainWidgetVisible(aIsParentWindowMainWidgetVisible); } \
  NS_IMETHOD IsNodeDisabledForEvents(nsINode *aNode, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsNodeDisabledForEvents(aNode, _retval); } \
  NS_IMETHOD GetPaintFlashing(bool *aPaintFlashing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaintFlashing(aPaintFlashing); } \
  NS_IMETHOD SetPaintFlashing(bool aPaintFlashing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPaintFlashing(aPaintFlashing); } \
  NS_IMETHOD GetOMTAStyle(mozilla::dom::Element *aElement, const nsAString& aProperty, const nsAString& aPseudoElement, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOMTAStyle(aElement, aProperty, aPseudoElement, _retval); } \
  NS_IMETHOD GetOMTCTransform(mozilla::dom::Element *aElement, const nsAString& aPseudoElement, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOMTCTransform(aElement, aPseudoElement, _retval); } \
  NS_IMETHOD IsAnimationInPendingTracker(mozilla::dom::Animation *aAnimation, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsAnimationInPendingTracker(aAnimation, _retval); } \
  NS_IMETHOD SetHandlingUserInput(bool aHandlingInput, nsIJSRAIIHelper **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHandlingUserInput(aHandlingInput, _retval); } \
  NS_IMETHOD GetContentAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentAPZTestData(cx, _retval); } \
  NS_IMETHOD GetCompositorAPZTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompositorAPZTestData(cx, _retval); } \
  NS_IMETHOD PostRestyleSelfEvent(mozilla::dom::Element *aElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PostRestyleSelfEvent(aElement); } \
  NS_IMETHOD XpconnectArgument(nsISupports *aObj) override { return !_to ? NS_ERROR_NULL_POINTER : _to->XpconnectArgument(aObj); } \
  NS_IMETHOD AskPermission(nsIContentPermissionRequest *aRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AskPermission(aRequest); } \
  NS_IMETHOD GetRestyleGeneration(uint64_t *aRestyleGeneration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRestyleGeneration(aRestyleGeneration); } \
  NS_IMETHOD GetFramesConstructed(uint64_t *aFramesConstructed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFramesConstructed(aFramesConstructed); } \
  NS_IMETHOD GetFramesReflowed(uint64_t *aFramesReflowed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFramesReflowed(aFramesReflowed); } \
  NS_IMETHOD SetChromeMargin(int32_t aTop, int32_t aRight, int32_t aBottom, int32_t aLeft) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChromeMargin(aTop, aRight, aBottom, aLeft); } \
  NS_IMETHOD GetFrameUniformityTestData(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFrameUniformityTestData(cx, _retval); } \
  NS_IMETHOD EnterChaosMode(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnterChaosMode(); } \
  NS_IMETHOD LeaveChaosMode(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LeaveChaosMode(); } \
  NS_IMETHOD TriggerDeviceReset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TriggerDeviceReset(); } \
  NS_IMETHOD HasRuleProcessorUsedByMultipleStyleSets(uint32_t aSheetType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasRuleProcessorUsedByMultipleStyleSets(aSheetType, _retval); } \
  NS_IMETHOD RespectDisplayPortSuppression(bool aEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RespectDisplayPortSuppression(aEnabled); } \
  NS_IMETHOD ForceReflowInterrupt(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceReflowInterrupt(); } \
  NS_IMETHOD TerminateGPUProcess(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->TerminateGPUProcess(); } \
  NS_IMETHOD GetGpuProcessPid(int32_t *aGpuProcessPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGpuProcessPid(aGpuProcessPid); } \
  NS_IMETHOD AddManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddManuallyManagedState(element, state); } \
  NS_IMETHOD RemoveManuallyManagedState(mozilla::dom::Element *element, const nsAString& state) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveManuallyManagedState(element, state); } \
  NS_IMETHOD GetStorageUsage(mozilla::dom::Storage *aStorage, int64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageUsage(aStorage, _retval); } \
  NS_IMETHOD GetDirectionFromText(const nsAString& aString, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDirectionFromText(aString, _retval); } \
  NS_IMETHOD EnsureDirtyRootFrame(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureDirtyRootFrame(); } \
  NS_IMETHOD WrCapture(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrCapture(); } \
  NS_IMETHOD WrToggleCaptureSequence(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WrToggleCaptureSequence(); } \
  NS_IMETHOD SetCompositionRecording(bool aValue, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCompositionRecording(aValue, _retval); } \
  NS_IMETHOD StartCompositionRecording(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartCompositionRecording(_retval); } \
  NS_IMETHOD StopCompositionRecording(bool aWriteToDisk, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopCompositionRecording(aWriteToDisk, _retval); } \
  NS_IMETHOD IsCssPropertyRecordedInUseCounter(const nsACString& aProperty, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCssPropertyRecordedInUseCounter(aProperty, _retval); } \
  NS_IMETHOD ResetMobileViewportManager(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetMobileViewportManager(); } \
  NS_IMETHOD GetSystemFont(nsACString& aSystemFont) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemFont(aSystemFont); } \
  NS_IMETHOD SetSystemFont(const nsACString& aSystemFont) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSystemFont(aSystemFont); } \
  NS_IMETHOD GetPaintCount(uint64_t *aPaintCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPaintCount(aPaintCount); } \
  NS_IMETHOD SyncFlushCompositor(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SyncFlushCompositor(); } \
  NS_IMETHOD GetLayersId(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLayersId(_retval); } \
  NS_IMETHOD GetUsesOverlayScrollbars(bool *aUsesOverlayScrollbars) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsesOverlayScrollbars(aUsesOverlayScrollbars); } \
  NS_IMETHOD GetEffectivelyThrottlesFrameRequests(bool *aEffectivelyThrottlesFrameRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEffectivelyThrottlesFrameRequests(aEffectivelyThrottlesFrameRequests); } \
  NS_IMETHOD GetWebrtcRawDeviceId(nsAString& aWebrtcRawDeviceId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWebrtcRawDeviceId(aWebrtcRawDeviceId); } 


/* starting interface:    nsITranslationNodeList */
#define NS_ITRANSLATIONNODELIST_IID_STR "c694e359-7227-4392-a138-33c0cc1f15a6"

#define NS_ITRANSLATIONNODELIST_IID \
  {0xc694e359, 0x7227, 0x4392, \
    { 0xa1, 0x38, 0x33, 0xc0, 0xcc, 0x1f, 0x15, 0xa6 }}

class NS_NO_VTABLE nsITranslationNodeList : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRANSLATIONNODELIST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITranslationNodeList;

  /* readonly attribute unsigned long length; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLength(uint32_t *aLength) = 0;

  /* Node item (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Item(uint32_t index, nsINode **_retval) = 0;

  /* boolean isTranslationRootAtIndex (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsTranslationRootAtIndex(uint32_t index, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITranslationNodeList, NS_ITRANSLATIONNODELIST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRANSLATIONNODELIST \
  NS_IMETHOD GetLength(uint32_t *aLength) override; \
  NS_IMETHOD Item(uint32_t index, nsINode **_retval) override; \
  NS_IMETHOD IsTranslationRootAtIndex(uint32_t index, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRANSLATIONNODELIST \
  nsresult GetLength(uint32_t *aLength); \
  nsresult Item(uint32_t index, nsINode **_retval); \
  nsresult IsTranslationRootAtIndex(uint32_t index, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRANSLATIONNODELIST(_to) \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return _to GetLength(aLength); } \
  NS_IMETHOD Item(uint32_t index, nsINode **_retval) override { return _to Item(index, _retval); } \
  NS_IMETHOD IsTranslationRootAtIndex(uint32_t index, bool *_retval) override { return _to IsTranslationRootAtIndex(index, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRANSLATIONNODELIST(_to) \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } \
  NS_IMETHOD Item(uint32_t index, nsINode **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Item(index, _retval); } \
  NS_IMETHOD IsTranslationRootAtIndex(uint32_t index, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsTranslationRootAtIndex(index, _retval); } 


/* starting interface:    nsIJSRAIIHelper */
#define NS_IJSRAIIHELPER_IID_STR "52e5a996-d0a9-4efc-a6fa-24489c532b19"

#define NS_IJSRAIIHELPER_IID \
  {0x52e5a996, 0xd0a9, 0x4efc, \
    { 0xa6, 0xfa, 0x24, 0x48, 0x9c, 0x53, 0x2b, 0x19 }}

class NS_NO_VTABLE nsIJSRAIIHelper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJSRAIIHELPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJSRAIIHelper;

  /* void destruct (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Destruct(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJSRAIIHelper, NS_IJSRAIIHELPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJSRAIIHELPER \
  NS_IMETHOD Destruct(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJSRAIIHELPER \
  nsresult Destruct(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJSRAIIHELPER(_to) \
  NS_IMETHOD Destruct(void) override { return _to Destruct(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJSRAIIHELPER(_to) \
  NS_IMETHOD Destruct(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Destruct(); } 


#endif /* __gen_nsIDOMWindowUtils_h__ */
