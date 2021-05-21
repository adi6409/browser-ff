/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIFocusManager.idl
 */

#ifndef __gen_nsIFocusManager_h__
#define __gen_nsIFocusManager_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIFocusManager */
#define NS_IFOCUSMANAGER_IID_STR "86e1f1e1-365d-493b-b52a-a649f3f311dc"

#define NS_IFOCUSMANAGER_IID \
  {0x86e1f1e1, 0x365d, 0x493b, \
    { 0xb5, 0x2a, 0xa6, 0x49, 0xf3, 0xf3, 0x11, 0xdc }}

class NS_NO_VTABLE nsIFocusManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFOCUSMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFocusManager;

  /* readonly attribute mozIDOMWindowProxy activeWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) = 0;

  /* readonly attribute BrowsingContext activeBrowsingContext; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActiveBrowsingContext(mozilla::dom::BrowsingContext **aActiveBrowsingContext) = 0;

  /* attribute mozIDOMWindowProxy focusedWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) = 0;

  /* readonly attribute BrowsingContext focusedContentBrowsingContext; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedContentBrowsingContext(mozilla::dom::BrowsingContext **aFocusedContentBrowsingContext) = 0;

  /* readonly attribute Element focusedElement; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) = 0;

  /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastFocusMethod(mozIDOMWindowProxy *window, uint32_t *_retval) = 0;

  /* [can_run_script] void setFocus (in Element aElement, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(mozilla::dom::Element *aElement, uint32_t aFlags) = 0;

  /* Element moveFocus (in mozIDOMWindowProxy aWindow, in Element aStartElement, in unsigned long aType, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveFocus(mozIDOMWindowProxy *aWindow, mozilla::dom::Element *aStartElement, uint32_t aType, uint32_t aFlags, mozilla::dom::Element **_retval) = 0;

  /* void clearFocus (in mozIDOMWindowProxy aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearFocus(mozIDOMWindowProxy *aWindow) = 0;

  /* Element getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedElementForWindow(mozIDOMWindowProxy *aWindow, bool aDeep, mozIDOMWindowProxy **aFocusedWindow, mozilla::dom::Element **_retval) = 0;

  /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveCaretToFocus(mozIDOMWindowProxy *aWindow) = 0;

  /* boolean elementIsFocusable (in Element aElement, in unsigned long aFlags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ElementIsFocusable(mozilla::dom::Element *aElement, uint32_t aFlags, bool *_retval) = 0;

  enum {
    FLAG_RAISE = 1U,
    FLAG_NOSCROLL = 2U,
    FLAG_NOSWITCHFRAME = 4U,
    FLAG_NOPARENTFRAME = 8U,
    FLAG_NONSYSTEMCALLER = 16U,
    FLAG_BYMOUSE = 4096U,
    FLAG_BYKEY = 8192U,
    FLAG_BYMOVEFOCUS = 16384U,
    FLAG_SHOWRING = 1048576U,
    FLAG_BYTOUCH = 2097152U,
    FLAG_BYLONGPRESS = 8388608U,
    MOVEFOCUS_FORWARD = 1U,
    MOVEFOCUS_BACKWARD = 2U,
    MOVEFOCUS_FORWARDDOC = 3U,
    MOVEFOCUS_BACKWARDDOC = 4U,
    MOVEFOCUS_FIRST = 5U,
    MOVEFOCUS_LAST = 6U,
    MOVEFOCUS_ROOT = 7U,
    MOVEFOCUS_CARET = 8U,
    MOVEFOCUS_FIRSTDOC = 9U,
    MOVEFOCUS_LASTDOC = 10U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFocusManager, NS_IFOCUSMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFOCUSMANAGER \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override; \
  NS_IMETHOD GetActiveBrowsingContext(mozilla::dom::BrowsingContext **aActiveBrowsingContext) override; \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override; \
  NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override; \
  NS_IMETHOD GetFocusedContentBrowsingContext(mozilla::dom::BrowsingContext **aFocusedContentBrowsingContext) override; \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override; \
  NS_IMETHOD GetLastFocusMethod(mozIDOMWindowProxy *window, uint32_t *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(mozilla::dom::Element *aElement, uint32_t aFlags) override; \
  NS_IMETHOD MoveFocus(mozIDOMWindowProxy *aWindow, mozilla::dom::Element *aStartElement, uint32_t aType, uint32_t aFlags, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD ClearFocus(mozIDOMWindowProxy *aWindow) override; \
  NS_IMETHOD GetFocusedElementForWindow(mozIDOMWindowProxy *aWindow, bool aDeep, mozIDOMWindowProxy **aFocusedWindow, mozilla::dom::Element **_retval) override; \
  NS_IMETHOD MoveCaretToFocus(mozIDOMWindowProxy *aWindow) override; \
  NS_IMETHOD ElementIsFocusable(mozilla::dom::Element *aElement, uint32_t aFlags, bool *_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFOCUSMANAGER \
  nsresult GetActiveWindow(mozIDOMWindowProxy **aActiveWindow); \
  nsresult GetActiveBrowsingContext(mozilla::dom::BrowsingContext **aActiveBrowsingContext); \
  nsresult GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow); \
  nsresult SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow); \
  nsresult GetFocusedContentBrowsingContext(mozilla::dom::BrowsingContext **aFocusedContentBrowsingContext); \
  nsresult GetFocusedElement(mozilla::dom::Element **aFocusedElement); \
  nsresult GetLastFocusMethod(mozIDOMWindowProxy *window, uint32_t *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult SetFocus(mozilla::dom::Element *aElement, uint32_t aFlags); \
  nsresult MoveFocus(mozIDOMWindowProxy *aWindow, mozilla::dom::Element *aStartElement, uint32_t aType, uint32_t aFlags, mozilla::dom::Element **_retval); \
  nsresult ClearFocus(mozIDOMWindowProxy *aWindow); \
  nsresult GetFocusedElementForWindow(mozIDOMWindowProxy *aWindow, bool aDeep, mozIDOMWindowProxy **aFocusedWindow, mozilla::dom::Element **_retval); \
  nsresult MoveCaretToFocus(mozIDOMWindowProxy *aWindow); \
  nsresult ElementIsFocusable(mozilla::dom::Element *aElement, uint32_t aFlags, bool *_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFOCUSMANAGER(_to) \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override { return _to GetActiveWindow(aActiveWindow); } \
  NS_IMETHOD GetActiveBrowsingContext(mozilla::dom::BrowsingContext **aActiveBrowsingContext) override { return _to GetActiveBrowsingContext(aActiveBrowsingContext); } \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override { return _to GetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override { return _to SetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD GetFocusedContentBrowsingContext(mozilla::dom::BrowsingContext **aFocusedContentBrowsingContext) override { return _to GetFocusedContentBrowsingContext(aFocusedContentBrowsingContext); } \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override { return _to GetFocusedElement(aFocusedElement); } \
  NS_IMETHOD GetLastFocusMethod(mozIDOMWindowProxy *window, uint32_t *_retval) override { return _to GetLastFocusMethod(window, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(mozilla::dom::Element *aElement, uint32_t aFlags) override { return _to SetFocus(aElement, aFlags); } \
  NS_IMETHOD MoveFocus(mozIDOMWindowProxy *aWindow, mozilla::dom::Element *aStartElement, uint32_t aType, uint32_t aFlags, mozilla::dom::Element **_retval) override { return _to MoveFocus(aWindow, aStartElement, aType, aFlags, _retval); } \
  NS_IMETHOD ClearFocus(mozIDOMWindowProxy *aWindow) override { return _to ClearFocus(aWindow); } \
  NS_IMETHOD GetFocusedElementForWindow(mozIDOMWindowProxy *aWindow, bool aDeep, mozIDOMWindowProxy **aFocusedWindow, mozilla::dom::Element **_retval) override { return _to GetFocusedElementForWindow(aWindow, aDeep, aFocusedWindow, _retval); } \
  NS_IMETHOD MoveCaretToFocus(mozIDOMWindowProxy *aWindow) override { return _to MoveCaretToFocus(aWindow); } \
  NS_IMETHOD ElementIsFocusable(mozilla::dom::Element *aElement, uint32_t aFlags, bool *_retval) override { return _to ElementIsFocusable(aElement, aFlags, _retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFOCUSMANAGER(_to) \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveWindow(aActiveWindow); } \
  NS_IMETHOD GetActiveBrowsingContext(mozilla::dom::BrowsingContext **aActiveBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveBrowsingContext(aActiveBrowsingContext); } \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD GetFocusedContentBrowsingContext(mozilla::dom::BrowsingContext **aFocusedContentBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedContentBrowsingContext(aFocusedContentBrowsingContext); } \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedElement(aFocusedElement); } \
  NS_IMETHOD GetLastFocusMethod(mozIDOMWindowProxy *window, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastFocusMethod(window, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocus(mozilla::dom::Element *aElement, uint32_t aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocus(aElement, aFlags); } \
  NS_IMETHOD MoveFocus(mozIDOMWindowProxy *aWindow, mozilla::dom::Element *aStartElement, uint32_t aType, uint32_t aFlags, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveFocus(aWindow, aStartElement, aType, aFlags, _retval); } \
  NS_IMETHOD ClearFocus(mozIDOMWindowProxy *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearFocus(aWindow); } \
  NS_IMETHOD GetFocusedElementForWindow(mozIDOMWindowProxy *aWindow, bool aDeep, mozIDOMWindowProxy **aFocusedWindow, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedElementForWindow(aWindow, aDeep, aFocusedWindow, _retval); } \
  NS_IMETHOD MoveCaretToFocus(mozIDOMWindowProxy *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveCaretToFocus(aWindow); } \
  NS_IMETHOD ElementIsFocusable(mozilla::dom::Element *aElement, uint32_t aFlags, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ElementIsFocusable(aElement, aFlags, _retval); } \


#endif /* __gen_nsIFocusManager_h__ */
