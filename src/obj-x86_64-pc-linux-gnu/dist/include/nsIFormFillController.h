/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormFillController.idl
 */

#ifndef __gen_nsIFormFillController_h__
#define __gen_nsIFormFillController_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompletePopup; /* forward declaration */

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

namespace mozilla {
namespace dom {
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class HTMLInputElement; /* webidl HTMLInputElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIFormFillController */
#define NS_IFORMFILLCONTROLLER_IID_STR "07f0a0dc-f6e9-4cdd-a55f-56d770523a4c"

#define NS_IFORMFILLCONTROLLER_IID \
  {0x07f0a0dc, 0xf6e9, 0x4cdd, \
    { 0xa5, 0x5f, 0x56, 0xd7, 0x70, 0x52, 0x3a, 0x4c }}

class NS_NO_VTABLE nsIFormFillController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMFILLCONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormFillController;

  /* readonly attribute HTMLInputElement focusedInput; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedInput(mozilla::dom::HTMLInputElement **aFocusedInput) = 0;

  /* readonly attribute boolean passwordPopupAutomaticallyOpened; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPasswordPopupAutomaticallyOpened(bool *aPasswordPopupAutomaticallyOpened) = 0;

  /* void attachPopupElementToDocument (in Document document, in Element popup); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AttachPopupElementToDocument(mozilla::dom::Document *document, mozilla::dom::Element *popup) = 0;

  /* void detachFromDocument (in Document document); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DetachFromDocument(mozilla::dom::Document *document) = 0;

  /* [can_run_script] void markAsLoginManagerField (in HTMLInputElement aInput); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsLoginManagerField(mozilla::dom::HTMLInputElement *aInput) = 0;

  /* [can_run_script] void markAsAutofillField (in HTMLInputElement aInput); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsAutofillField(mozilla::dom::HTMLInputElement *aInput) = 0;

  /* [can_run_script] void showPopup (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPopup(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormFillController, NS_IFORMFILLCONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMFILLCONTROLLER \
  NS_IMETHOD GetFocusedInput(mozilla::dom::HTMLInputElement **aFocusedInput) override; \
  NS_IMETHOD GetPasswordPopupAutomaticallyOpened(bool *aPasswordPopupAutomaticallyOpened) override; \
  NS_IMETHOD AttachPopupElementToDocument(mozilla::dom::Document *document, mozilla::dom::Element *popup) override; \
  NS_IMETHOD DetachFromDocument(mozilla::dom::Document *document) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsLoginManagerField(mozilla::dom::HTMLInputElement *aInput) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsAutofillField(mozilla::dom::HTMLInputElement *aInput) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPopup(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMFILLCONTROLLER \
  nsresult GetFocusedInput(mozilla::dom::HTMLInputElement **aFocusedInput); \
  nsresult GetPasswordPopupAutomaticallyOpened(bool *aPasswordPopupAutomaticallyOpened); \
  nsresult AttachPopupElementToDocument(mozilla::dom::Document *document, mozilla::dom::Element *popup); \
  nsresult DetachFromDocument(mozilla::dom::Document *document); \
  MOZ_CAN_RUN_SCRIPT nsresult MarkAsLoginManagerField(mozilla::dom::HTMLInputElement *aInput); \
  MOZ_CAN_RUN_SCRIPT nsresult MarkAsAutofillField(mozilla::dom::HTMLInputElement *aInput); \
  MOZ_CAN_RUN_SCRIPT nsresult ShowPopup(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMFILLCONTROLLER(_to) \
  NS_IMETHOD GetFocusedInput(mozilla::dom::HTMLInputElement **aFocusedInput) override { return _to GetFocusedInput(aFocusedInput); } \
  NS_IMETHOD GetPasswordPopupAutomaticallyOpened(bool *aPasswordPopupAutomaticallyOpened) override { return _to GetPasswordPopupAutomaticallyOpened(aPasswordPopupAutomaticallyOpened); } \
  NS_IMETHOD AttachPopupElementToDocument(mozilla::dom::Document *document, mozilla::dom::Element *popup) override { return _to AttachPopupElementToDocument(document, popup); } \
  NS_IMETHOD DetachFromDocument(mozilla::dom::Document *document) override { return _to DetachFromDocument(document); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsLoginManagerField(mozilla::dom::HTMLInputElement *aInput) override { return _to MarkAsLoginManagerField(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsAutofillField(mozilla::dom::HTMLInputElement *aInput) override { return _to MarkAsAutofillField(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPopup(void) override { return _to ShowPopup(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMFILLCONTROLLER(_to) \
  NS_IMETHOD GetFocusedInput(mozilla::dom::HTMLInputElement **aFocusedInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedInput(aFocusedInput); } \
  NS_IMETHOD GetPasswordPopupAutomaticallyOpened(bool *aPasswordPopupAutomaticallyOpened) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPasswordPopupAutomaticallyOpened(aPasswordPopupAutomaticallyOpened); } \
  NS_IMETHOD AttachPopupElementToDocument(mozilla::dom::Document *document, mozilla::dom::Element *popup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AttachPopupElementToDocument(document, popup); } \
  NS_IMETHOD DetachFromDocument(mozilla::dom::Document *document) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DetachFromDocument(document); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsLoginManagerField(mozilla::dom::HTMLInputElement *aInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkAsLoginManagerField(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD MarkAsAutofillField(mozilla::dom::HTMLInputElement *aInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MarkAsAutofillField(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ShowPopup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowPopup(); } 


#endif /* __gen_nsIFormFillController_h__ */
