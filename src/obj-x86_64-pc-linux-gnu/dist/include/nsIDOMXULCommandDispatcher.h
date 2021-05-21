/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULCommandDispatcher.idl
 */

#ifndef __gen_nsIDOMXULCommandDispatcher_h__
#define __gen_nsIDOMXULCommandDispatcher_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIController; /* forward declaration */

class nsIControllers; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULCommandDispatcher */
#define NS_IDOMXULCOMMANDDISPATCHER_IID_STR "a9fa9fd3-8d62-4f94-9ed8-3ea9c3cf0773"

#define NS_IDOMXULCOMMANDDISPATCHER_IID \
  {0xa9fa9fd3, 0x8d62, 0x4f94, \
    { 0x9e, 0xd8, 0x3e, 0xa9, 0xc3, 0xcf, 0x07, 0x73 }}

class NS_NO_VTABLE nsIDOMXULCommandDispatcher : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULCOMMANDDISPATCHER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULCommandDispatcher;

  /* [setter_can_run_script] attribute Element focusedElement; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) = 0;
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedElement(mozilla::dom::Element *aFocusedElement) = 0;

  /* [setter_can_run_script] attribute mozIDOMWindowProxy focusedWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) = 0;
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) = 0;

  /* void addCommandUpdater (in Element updater, in AString events, in AString targets); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddCommandUpdater(mozilla::dom::Element *updater, const nsAString& events, const nsAString& targets) = 0;

  /* void removeCommandUpdater (in Element updater); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveCommandUpdater(mozilla::dom::Element *updater) = 0;

  /* void updateCommands (in AString eventName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateCommands(const nsAString& eventName) = 0;

  /* nsIController getControllerForCommand (in string command); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) = 0;

  /* nsIControllers getControllers (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetControllers(nsIControllers **_retval) = 0;

  /* void advanceFocus (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AdvanceFocus(void) = 0;

  /* void rewindFocus (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RewindFocus(void) = 0;

  /* void advanceFocusIntoSubtree (in Element elt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AdvanceFocusIntoSubtree(mozilla::dom::Element *elt) = 0;

  /* void lock (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Lock(void) = 0;

  /* void unlock (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Unlock(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULCommandDispatcher, NS_IDOMXULCOMMANDDISPATCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULCOMMANDDISPATCHER \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedElement(mozilla::dom::Element *aFocusedElement) override; \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override; \
  NS_IMETHOD AddCommandUpdater(mozilla::dom::Element *updater, const nsAString& events, const nsAString& targets) override; \
  NS_IMETHOD RemoveCommandUpdater(mozilla::dom::Element *updater) override; \
  NS_IMETHOD UpdateCommands(const nsAString& eventName) override; \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override; \
  NS_IMETHOD GetControllers(nsIControllers **_retval) override; \
  NS_IMETHOD AdvanceFocus(void) override; \
  NS_IMETHOD RewindFocus(void) override; \
  NS_IMETHOD AdvanceFocusIntoSubtree(mozilla::dom::Element *elt) override; \
  NS_IMETHOD Lock(void) override; \
  NS_IMETHOD Unlock(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULCOMMANDDISPATCHER \
  nsresult GetFocusedElement(mozilla::dom::Element **aFocusedElement); \
  MOZ_CAN_RUN_SCRIPT nsresult SetFocusedElement(mozilla::dom::Element *aFocusedElement); \
  nsresult GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow); \
  MOZ_CAN_RUN_SCRIPT nsresult SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow); \
  nsresult AddCommandUpdater(mozilla::dom::Element *updater, const nsAString& events, const nsAString& targets); \
  nsresult RemoveCommandUpdater(mozilla::dom::Element *updater); \
  nsresult UpdateCommands(const nsAString& eventName); \
  nsresult GetControllerForCommand(const char * command, nsIController **_retval); \
  nsresult GetControllers(nsIControllers **_retval); \
  nsresult AdvanceFocus(void); \
  nsresult RewindFocus(void); \
  nsresult AdvanceFocusIntoSubtree(mozilla::dom::Element *elt); \
  nsresult Lock(void); \
  nsresult Unlock(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULCOMMANDDISPATCHER(_to) \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override { return _to GetFocusedElement(aFocusedElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedElement(mozilla::dom::Element *aFocusedElement) override { return _to SetFocusedElement(aFocusedElement); } \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override { return _to GetFocusedWindow(aFocusedWindow); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override { return _to SetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD AddCommandUpdater(mozilla::dom::Element *updater, const nsAString& events, const nsAString& targets) override { return _to AddCommandUpdater(updater, events, targets); } \
  NS_IMETHOD RemoveCommandUpdater(mozilla::dom::Element *updater) override { return _to RemoveCommandUpdater(updater); } \
  NS_IMETHOD UpdateCommands(const nsAString& eventName) override { return _to UpdateCommands(eventName); } \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override { return _to GetControllerForCommand(command, _retval); } \
  NS_IMETHOD GetControllers(nsIControllers **_retval) override { return _to GetControllers(_retval); } \
  NS_IMETHOD AdvanceFocus(void) override { return _to AdvanceFocus(); } \
  NS_IMETHOD RewindFocus(void) override { return _to RewindFocus(); } \
  NS_IMETHOD AdvanceFocusIntoSubtree(mozilla::dom::Element *elt) override { return _to AdvanceFocusIntoSubtree(elt); } \
  NS_IMETHOD Lock(void) override { return _to Lock(); } \
  NS_IMETHOD Unlock(void) override { return _to Unlock(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULCOMMANDDISPATCHER(_to) \
  NS_IMETHOD GetFocusedElement(mozilla::dom::Element **aFocusedElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedElement(aFocusedElement); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedElement(mozilla::dom::Element *aFocusedElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocusedElement(aFocusedElement); } \
  NS_IMETHOD GetFocusedWindow(mozIDOMWindowProxy **aFocusedWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedWindow(aFocusedWindow); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedWindow(mozIDOMWindowProxy *aFocusedWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocusedWindow(aFocusedWindow); } \
  NS_IMETHOD AddCommandUpdater(mozilla::dom::Element *updater, const nsAString& events, const nsAString& targets) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddCommandUpdater(updater, events, targets); } \
  NS_IMETHOD RemoveCommandUpdater(mozilla::dom::Element *updater) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveCommandUpdater(updater); } \
  NS_IMETHOD UpdateCommands(const nsAString& eventName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateCommands(eventName); } \
  NS_IMETHOD GetControllerForCommand(const char * command, nsIController **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllerForCommand(command, _retval); } \
  NS_IMETHOD GetControllers(nsIControllers **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetControllers(_retval); } \
  NS_IMETHOD AdvanceFocus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdvanceFocus(); } \
  NS_IMETHOD RewindFocus(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RewindFocus(); } \
  NS_IMETHOD AdvanceFocusIntoSubtree(mozilla::dom::Element *elt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AdvanceFocusIntoSubtree(elt); } \
  NS_IMETHOD Lock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Lock(); } \
  NS_IMETHOD Unlock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unlock(); } 


#endif /* __gen_nsIDOMXULCommandDispatcher_h__ */
