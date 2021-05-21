/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompletePopup.idl
 */

#ifndef __gen_nsIAutoCompletePopup_h__
#define __gen_nsIAutoCompletePopup_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompleteInput; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIAutoCompletePopup */
#define NS_IAUTOCOMPLETEPOPUP_IID_STR "bd3c2662-a988-41ab-8c94-c15ed0e6ac7d"

#define NS_IAUTOCOMPLETEPOPUP_IID \
  {0xbd3c2662, 0xa988, 0x41ab, \
    { 0x8c, 0x94, 0xc1, 0x5e, 0xd0, 0xe6, 0xac, 0x7d }}

class NS_NO_VTABLE nsIAutoCompletePopup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETEPOPUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompletePopup;

  /* readonly attribute nsIAutoCompleteInput input; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) = 0;

  /* readonly attribute AString overrideValue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOverrideValue(nsAString& aOverrideValue) = 0;

  /* attribute long selectedIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) = 0;

  /* readonly attribute boolean popupOpen; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupOpen(bool *aPopupOpen) = 0;

  /* void openAutocompletePopup (in nsIAutoCompleteInput input, in Element element); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenAutocompletePopup(nsIAutoCompleteInput *input, mozilla::dom::Element *element) = 0;

  /* void closePopup (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClosePopup(void) = 0;

  /* void invalidate (in unsigned short reason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Invalidate(uint16_t reason) = 0;

  enum {
    INVALIDATE_REASON_NEW_RESULT = 0U,
    INVALIDATE_REASON_DELETE = 1U
  };

  /* void selectBy (in boolean reverse, in boolean page); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectBy(bool reverse, bool page) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompletePopup, NS_IAUTOCOMPLETEPOPUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETEPOPUP \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override; \
  NS_IMETHOD GetOverrideValue(nsAString& aOverrideValue) override; \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override; \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override; \
  NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override; \
  NS_IMETHOD OpenAutocompletePopup(nsIAutoCompleteInput *input, mozilla::dom::Element *element) override; \
  NS_IMETHOD ClosePopup(void) override; \
  NS_IMETHOD Invalidate(uint16_t reason) override; \
  NS_IMETHOD SelectBy(bool reverse, bool page) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETEPOPUP \
  nsresult GetInput(nsIAutoCompleteInput **aInput); \
  nsresult GetOverrideValue(nsAString& aOverrideValue); \
  nsresult GetSelectedIndex(int32_t *aSelectedIndex); \
  nsresult SetSelectedIndex(int32_t aSelectedIndex); \
  nsresult GetPopupOpen(bool *aPopupOpen); \
  nsresult OpenAutocompletePopup(nsIAutoCompleteInput *input, mozilla::dom::Element *element); \
  nsresult ClosePopup(void); \
  nsresult Invalidate(uint16_t reason); \
  nsresult SelectBy(bool reverse, bool page); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETEPOPUP(_to) \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override { return _to GetInput(aInput); } \
  NS_IMETHOD GetOverrideValue(nsAString& aOverrideValue) override { return _to GetOverrideValue(aOverrideValue); } \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override { return _to GetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override { return _to SetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override { return _to GetPopupOpen(aPopupOpen); } \
  NS_IMETHOD OpenAutocompletePopup(nsIAutoCompleteInput *input, mozilla::dom::Element *element) override { return _to OpenAutocompletePopup(input, element); } \
  NS_IMETHOD ClosePopup(void) override { return _to ClosePopup(); } \
  NS_IMETHOD Invalidate(uint16_t reason) override { return _to Invalidate(reason); } \
  NS_IMETHOD SelectBy(bool reverse, bool page) override { return _to SelectBy(reverse, page); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETEPOPUP(_to) \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInput(aInput); } \
  NS_IMETHOD GetOverrideValue(nsAString& aOverrideValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOverrideValue(aOverrideValue); } \
  NS_IMETHOD GetSelectedIndex(int32_t *aSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD SetSelectedIndex(int32_t aSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectedIndex(aSelectedIndex); } \
  NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPopupOpen(aPopupOpen); } \
  NS_IMETHOD OpenAutocompletePopup(nsIAutoCompleteInput *input, mozilla::dom::Element *element) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenAutocompletePopup(input, element); } \
  NS_IMETHOD ClosePopup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClosePopup(); } \
  NS_IMETHOD Invalidate(uint16_t reason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Invalidate(reason); } \
  NS_IMETHOD SelectBy(bool reverse, bool page) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectBy(reverse, page); } 


#endif /* __gen_nsIAutoCompletePopup_h__ */
