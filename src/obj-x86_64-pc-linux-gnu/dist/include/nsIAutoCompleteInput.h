/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteInput.idl
 */

#ifndef __gen_nsIAutoCompleteInput_h__
#define __gen_nsIAutoCompleteInput_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIAutoCompleteController_h__
#include "nsIAutoCompleteController.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompletePopup; /* forward declaration */

namespace mozilla {
namespace dom {
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIAutoCompleteInput */
#define NS_IAUTOCOMPLETEINPUT_IID_STR "b068e70f-f82c-4c12-ad87-82e271c5c180"

#define NS_IAUTOCOMPLETEINPUT_IID \
  {0xb068e70f, 0xf82c, 0x4c12, \
    { 0xad, 0x87, 0x82, 0xe2, 0x71, 0xc5, 0xc1, 0x80 }}

class NS_NO_VTABLE nsIAutoCompleteInput : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETEINPUT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteInput;

  /* readonly attribute Element popupElement; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupElement(mozilla::dom::Element **aPopupElement) = 0;

  /* readonly attribute nsIAutoCompletePopup popup; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopup(nsIAutoCompletePopup **aPopup) = 0;

  /* readonly attribute nsIAutoCompleteController controller; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetController(nsIAutoCompleteController **aController) = 0;

  /* [can_run_script] attribute boolean popupOpen; */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupOpen(bool *aPopupOpen) = 0;
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetPopupOpen(bool aPopupOpen) = 0;

  /* attribute boolean disableAutoComplete; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisableAutoComplete(bool *aDisableAutoComplete) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisableAutoComplete(bool aDisableAutoComplete) = 0;

  /* attribute boolean completeDefaultIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCompleteDefaultIndex(bool *aCompleteDefaultIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCompleteDefaultIndex(bool aCompleteDefaultIndex) = 0;

  /* attribute boolean completeSelectedIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCompleteSelectedIndex(bool *aCompleteSelectedIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCompleteSelectedIndex(bool aCompleteSelectedIndex) = 0;

  /* attribute boolean forceComplete; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetForceComplete(bool *aForceComplete) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetForceComplete(bool aForceComplete) = 0;

  /* attribute unsigned long minResultsForPopup; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMinResultsForPopup(uint32_t *aMinResultsForPopup) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMinResultsForPopup(uint32_t aMinResultsForPopup) = 0;

  /* attribute unsigned long maxRows; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMaxRows(uint32_t *aMaxRows) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMaxRows(uint32_t aMaxRows) = 0;

  /* attribute unsigned long timeout; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTimeout(uint32_t *aTimeout) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTimeout(uint32_t aTimeout) = 0;

  /* attribute AString searchParam; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchParam(nsAString& aSearchParam) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchParam(const nsAString& aSearchParam) = 0;

  /* readonly attribute unsigned long searchCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchCount(uint32_t *aSearchCount) = 0;

  /* ACString getSearchAt (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchAt(uint32_t index, nsACString& _retval) = 0;

  /* attribute AString textValue; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTextValue(nsAString& aTextValue) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTextValue(const nsAString& aTextValue) = 0;

  enum {
    TEXTVALUE_REASON_UNKNOWN = 0U,
    TEXTVALUE_REASON_COMPLETEDEFAULT = 1U,
    TEXTVALUE_REASON_COMPLETESELECTED = 2U,
    TEXTVALUE_REASON_REVERT = 3U,
    TEXTVALUE_REASON_ENTERMATCH = 4U
  };

  /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTextValueWithReason(const nsAString& aValue, uint16_t aReason) = 0;

  /* readonly attribute long selectionStart; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectionStart(int32_t *aSelectionStart) = 0;

  /* readonly attribute long selectionEnd; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSelectionEnd(int32_t *aSelectionEnd) = 0;

  /* void selectTextRange (in long startIndex, in long endIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SelectTextRange(int32_t startIndex, int32_t endIndex) = 0;

  /* void onSearchBegin (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchBegin(void) = 0;

  /* void onSearchComplete (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchComplete(void) = 0;

  /* boolean onTextEntered ([optional] in Event aEvent, [optional] in boolean itemWasSelected); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTextEntered(mozilla::dom::Event *aEvent, bool itemWasSelected, bool *_retval) = 0;

  /* boolean onTextReverted (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnTextReverted(bool *_retval) = 0;

  /* readonly attribute boolean consumeRollupEvent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetConsumeRollupEvent(bool *aConsumeRollupEvent) = 0;

  /* readonly attribute boolean inPrivateContext; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInPrivateContext(bool *aInPrivateContext) = 0;

  /* readonly attribute boolean noRollupOnCaretMove; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNoRollupOnCaretMove(bool *aNoRollupOnCaretMove) = 0;

  /* readonly attribute boolean noRollupOnEmptySearch; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNoRollupOnEmptySearch(bool *aNoRollupOnEmptySearch) = 0;

  /* readonly attribute unsigned long userContextId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteInput, NS_IAUTOCOMPLETEINPUT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETEINPUT \
  NS_IMETHOD GetPopupElement(mozilla::dom::Element **aPopupElement) override; \
  NS_IMETHOD GetPopup(nsIAutoCompletePopup **aPopup) override; \
  NS_IMETHOD GetController(nsIAutoCompleteController **aController) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetPopupOpen(bool aPopupOpen) override; \
  NS_IMETHOD GetDisableAutoComplete(bool *aDisableAutoComplete) override; \
  NS_IMETHOD SetDisableAutoComplete(bool aDisableAutoComplete) override; \
  NS_IMETHOD GetCompleteDefaultIndex(bool *aCompleteDefaultIndex) override; \
  NS_IMETHOD SetCompleteDefaultIndex(bool aCompleteDefaultIndex) override; \
  NS_IMETHOD GetCompleteSelectedIndex(bool *aCompleteSelectedIndex) override; \
  NS_IMETHOD SetCompleteSelectedIndex(bool aCompleteSelectedIndex) override; \
  NS_IMETHOD GetForceComplete(bool *aForceComplete) override; \
  NS_IMETHOD SetForceComplete(bool aForceComplete) override; \
  NS_IMETHOD GetMinResultsForPopup(uint32_t *aMinResultsForPopup) override; \
  NS_IMETHOD SetMinResultsForPopup(uint32_t aMinResultsForPopup) override; \
  NS_IMETHOD GetMaxRows(uint32_t *aMaxRows) override; \
  NS_IMETHOD SetMaxRows(uint32_t aMaxRows) override; \
  NS_IMETHOD GetTimeout(uint32_t *aTimeout) override; \
  NS_IMETHOD SetTimeout(uint32_t aTimeout) override; \
  NS_IMETHOD GetSearchParam(nsAString& aSearchParam) override; \
  NS_IMETHOD SetSearchParam(const nsAString& aSearchParam) override; \
  NS_IMETHOD GetSearchCount(uint32_t *aSearchCount) override; \
  NS_IMETHOD GetSearchAt(uint32_t index, nsACString& _retval) override; \
  NS_IMETHOD GetTextValue(nsAString& aTextValue) override; \
  NS_IMETHOD SetTextValue(const nsAString& aTextValue) override; \
  NS_IMETHOD SetTextValueWithReason(const nsAString& aValue, uint16_t aReason) override; \
  NS_IMETHOD GetSelectionStart(int32_t *aSelectionStart) override; \
  NS_IMETHOD GetSelectionEnd(int32_t *aSelectionEnd) override; \
  NS_IMETHOD SelectTextRange(int32_t startIndex, int32_t endIndex) override; \
  NS_IMETHOD OnSearchBegin(void) override; \
  NS_IMETHOD OnSearchComplete(void) override; \
  NS_IMETHOD OnTextEntered(mozilla::dom::Event *aEvent, bool itemWasSelected, bool *_retval) override; \
  NS_IMETHOD OnTextReverted(bool *_retval) override; \
  NS_IMETHOD GetConsumeRollupEvent(bool *aConsumeRollupEvent) override; \
  NS_IMETHOD GetInPrivateContext(bool *aInPrivateContext) override; \
  NS_IMETHOD GetNoRollupOnCaretMove(bool *aNoRollupOnCaretMove) override; \
  NS_IMETHOD GetNoRollupOnEmptySearch(bool *aNoRollupOnEmptySearch) override; \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETEINPUT \
  nsresult GetPopupElement(mozilla::dom::Element **aPopupElement); \
  nsresult GetPopup(nsIAutoCompletePopup **aPopup); \
  nsresult GetController(nsIAutoCompleteController **aController); \
  MOZ_CAN_RUN_SCRIPT nsresult GetPopupOpen(bool *aPopupOpen); \
  MOZ_CAN_RUN_SCRIPT nsresult SetPopupOpen(bool aPopupOpen); \
  nsresult GetDisableAutoComplete(bool *aDisableAutoComplete); \
  nsresult SetDisableAutoComplete(bool aDisableAutoComplete); \
  nsresult GetCompleteDefaultIndex(bool *aCompleteDefaultIndex); \
  nsresult SetCompleteDefaultIndex(bool aCompleteDefaultIndex); \
  nsresult GetCompleteSelectedIndex(bool *aCompleteSelectedIndex); \
  nsresult SetCompleteSelectedIndex(bool aCompleteSelectedIndex); \
  nsresult GetForceComplete(bool *aForceComplete); \
  nsresult SetForceComplete(bool aForceComplete); \
  nsresult GetMinResultsForPopup(uint32_t *aMinResultsForPopup); \
  nsresult SetMinResultsForPopup(uint32_t aMinResultsForPopup); \
  nsresult GetMaxRows(uint32_t *aMaxRows); \
  nsresult SetMaxRows(uint32_t aMaxRows); \
  nsresult GetTimeout(uint32_t *aTimeout); \
  nsresult SetTimeout(uint32_t aTimeout); \
  nsresult GetSearchParam(nsAString& aSearchParam); \
  nsresult SetSearchParam(const nsAString& aSearchParam); \
  nsresult GetSearchCount(uint32_t *aSearchCount); \
  nsresult GetSearchAt(uint32_t index, nsACString& _retval); \
  nsresult GetTextValue(nsAString& aTextValue); \
  nsresult SetTextValue(const nsAString& aTextValue); \
  nsresult SetTextValueWithReason(const nsAString& aValue, uint16_t aReason); \
  nsresult GetSelectionStart(int32_t *aSelectionStart); \
  nsresult GetSelectionEnd(int32_t *aSelectionEnd); \
  nsresult SelectTextRange(int32_t startIndex, int32_t endIndex); \
  nsresult OnSearchBegin(void); \
  nsresult OnSearchComplete(void); \
  nsresult OnTextEntered(mozilla::dom::Event *aEvent, bool itemWasSelected, bool *_retval); \
  nsresult OnTextReverted(bool *_retval); \
  nsresult GetConsumeRollupEvent(bool *aConsumeRollupEvent); \
  nsresult GetInPrivateContext(bool *aInPrivateContext); \
  nsresult GetNoRollupOnCaretMove(bool *aNoRollupOnCaretMove); \
  nsresult GetNoRollupOnEmptySearch(bool *aNoRollupOnEmptySearch); \
  nsresult GetUserContextId(uint32_t *aUserContextId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETEINPUT(_to) \
  NS_IMETHOD GetPopupElement(mozilla::dom::Element **aPopupElement) override { return _to GetPopupElement(aPopupElement); } \
  NS_IMETHOD GetPopup(nsIAutoCompletePopup **aPopup) override { return _to GetPopup(aPopup); } \
  NS_IMETHOD GetController(nsIAutoCompleteController **aController) override { return _to GetController(aController); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override { return _to GetPopupOpen(aPopupOpen); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetPopupOpen(bool aPopupOpen) override { return _to SetPopupOpen(aPopupOpen); } \
  NS_IMETHOD GetDisableAutoComplete(bool *aDisableAutoComplete) override { return _to GetDisableAutoComplete(aDisableAutoComplete); } \
  NS_IMETHOD SetDisableAutoComplete(bool aDisableAutoComplete) override { return _to SetDisableAutoComplete(aDisableAutoComplete); } \
  NS_IMETHOD GetCompleteDefaultIndex(bool *aCompleteDefaultIndex) override { return _to GetCompleteDefaultIndex(aCompleteDefaultIndex); } \
  NS_IMETHOD SetCompleteDefaultIndex(bool aCompleteDefaultIndex) override { return _to SetCompleteDefaultIndex(aCompleteDefaultIndex); } \
  NS_IMETHOD GetCompleteSelectedIndex(bool *aCompleteSelectedIndex) override { return _to GetCompleteSelectedIndex(aCompleteSelectedIndex); } \
  NS_IMETHOD SetCompleteSelectedIndex(bool aCompleteSelectedIndex) override { return _to SetCompleteSelectedIndex(aCompleteSelectedIndex); } \
  NS_IMETHOD GetForceComplete(bool *aForceComplete) override { return _to GetForceComplete(aForceComplete); } \
  NS_IMETHOD SetForceComplete(bool aForceComplete) override { return _to SetForceComplete(aForceComplete); } \
  NS_IMETHOD GetMinResultsForPopup(uint32_t *aMinResultsForPopup) override { return _to GetMinResultsForPopup(aMinResultsForPopup); } \
  NS_IMETHOD SetMinResultsForPopup(uint32_t aMinResultsForPopup) override { return _to SetMinResultsForPopup(aMinResultsForPopup); } \
  NS_IMETHOD GetMaxRows(uint32_t *aMaxRows) override { return _to GetMaxRows(aMaxRows); } \
  NS_IMETHOD SetMaxRows(uint32_t aMaxRows) override { return _to SetMaxRows(aMaxRows); } \
  NS_IMETHOD GetTimeout(uint32_t *aTimeout) override { return _to GetTimeout(aTimeout); } \
  NS_IMETHOD SetTimeout(uint32_t aTimeout) override { return _to SetTimeout(aTimeout); } \
  NS_IMETHOD GetSearchParam(nsAString& aSearchParam) override { return _to GetSearchParam(aSearchParam); } \
  NS_IMETHOD SetSearchParam(const nsAString& aSearchParam) override { return _to SetSearchParam(aSearchParam); } \
  NS_IMETHOD GetSearchCount(uint32_t *aSearchCount) override { return _to GetSearchCount(aSearchCount); } \
  NS_IMETHOD GetSearchAt(uint32_t index, nsACString& _retval) override { return _to GetSearchAt(index, _retval); } \
  NS_IMETHOD GetTextValue(nsAString& aTextValue) override { return _to GetTextValue(aTextValue); } \
  NS_IMETHOD SetTextValue(const nsAString& aTextValue) override { return _to SetTextValue(aTextValue); } \
  NS_IMETHOD SetTextValueWithReason(const nsAString& aValue, uint16_t aReason) override { return _to SetTextValueWithReason(aValue, aReason); } \
  NS_IMETHOD GetSelectionStart(int32_t *aSelectionStart) override { return _to GetSelectionStart(aSelectionStart); } \
  NS_IMETHOD GetSelectionEnd(int32_t *aSelectionEnd) override { return _to GetSelectionEnd(aSelectionEnd); } \
  NS_IMETHOD SelectTextRange(int32_t startIndex, int32_t endIndex) override { return _to SelectTextRange(startIndex, endIndex); } \
  NS_IMETHOD OnSearchBegin(void) override { return _to OnSearchBegin(); } \
  NS_IMETHOD OnSearchComplete(void) override { return _to OnSearchComplete(); } \
  NS_IMETHOD OnTextEntered(mozilla::dom::Event *aEvent, bool itemWasSelected, bool *_retval) override { return _to OnTextEntered(aEvent, itemWasSelected, _retval); } \
  NS_IMETHOD OnTextReverted(bool *_retval) override { return _to OnTextReverted(_retval); } \
  NS_IMETHOD GetConsumeRollupEvent(bool *aConsumeRollupEvent) override { return _to GetConsumeRollupEvent(aConsumeRollupEvent); } \
  NS_IMETHOD GetInPrivateContext(bool *aInPrivateContext) override { return _to GetInPrivateContext(aInPrivateContext); } \
  NS_IMETHOD GetNoRollupOnCaretMove(bool *aNoRollupOnCaretMove) override { return _to GetNoRollupOnCaretMove(aNoRollupOnCaretMove); } \
  NS_IMETHOD GetNoRollupOnEmptySearch(bool *aNoRollupOnEmptySearch) override { return _to GetNoRollupOnEmptySearch(aNoRollupOnEmptySearch); } \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override { return _to GetUserContextId(aUserContextId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETEINPUT(_to) \
  NS_IMETHOD GetPopupElement(mozilla::dom::Element **aPopupElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPopupElement(aPopupElement); } \
  NS_IMETHOD GetPopup(nsIAutoCompletePopup **aPopup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPopup(aPopup); } \
  NS_IMETHOD GetController(nsIAutoCompleteController **aController) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetController(aController); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetPopupOpen(bool *aPopupOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPopupOpen(aPopupOpen); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetPopupOpen(bool aPopupOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPopupOpen(aPopupOpen); } \
  NS_IMETHOD GetDisableAutoComplete(bool *aDisableAutoComplete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisableAutoComplete(aDisableAutoComplete); } \
  NS_IMETHOD SetDisableAutoComplete(bool aDisableAutoComplete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisableAutoComplete(aDisableAutoComplete); } \
  NS_IMETHOD GetCompleteDefaultIndex(bool *aCompleteDefaultIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompleteDefaultIndex(aCompleteDefaultIndex); } \
  NS_IMETHOD SetCompleteDefaultIndex(bool aCompleteDefaultIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCompleteDefaultIndex(aCompleteDefaultIndex); } \
  NS_IMETHOD GetCompleteSelectedIndex(bool *aCompleteSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCompleteSelectedIndex(aCompleteSelectedIndex); } \
  NS_IMETHOD SetCompleteSelectedIndex(bool aCompleteSelectedIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCompleteSelectedIndex(aCompleteSelectedIndex); } \
  NS_IMETHOD GetForceComplete(bool *aForceComplete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceComplete(aForceComplete); } \
  NS_IMETHOD SetForceComplete(bool aForceComplete) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetForceComplete(aForceComplete); } \
  NS_IMETHOD GetMinResultsForPopup(uint32_t *aMinResultsForPopup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMinResultsForPopup(aMinResultsForPopup); } \
  NS_IMETHOD SetMinResultsForPopup(uint32_t aMinResultsForPopup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMinResultsForPopup(aMinResultsForPopup); } \
  NS_IMETHOD GetMaxRows(uint32_t *aMaxRows) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxRows(aMaxRows); } \
  NS_IMETHOD SetMaxRows(uint32_t aMaxRows) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMaxRows(aMaxRows); } \
  NS_IMETHOD GetTimeout(uint32_t *aTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTimeout(aTimeout); } \
  NS_IMETHOD SetTimeout(uint32_t aTimeout) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTimeout(aTimeout); } \
  NS_IMETHOD GetSearchParam(nsAString& aSearchParam) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchParam(aSearchParam); } \
  NS_IMETHOD SetSearchParam(const nsAString& aSearchParam) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchParam(aSearchParam); } \
  NS_IMETHOD GetSearchCount(uint32_t *aSearchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchCount(aSearchCount); } \
  NS_IMETHOD GetSearchAt(uint32_t index, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchAt(index, _retval); } \
  NS_IMETHOD GetTextValue(nsAString& aTextValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTextValue(aTextValue); } \
  NS_IMETHOD SetTextValue(const nsAString& aTextValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTextValue(aTextValue); } \
  NS_IMETHOD SetTextValueWithReason(const nsAString& aValue, uint16_t aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTextValueWithReason(aValue, aReason); } \
  NS_IMETHOD GetSelectionStart(int32_t *aSelectionStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionStart(aSelectionStart); } \
  NS_IMETHOD GetSelectionEnd(int32_t *aSelectionEnd) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionEnd(aSelectionEnd); } \
  NS_IMETHOD SelectTextRange(int32_t startIndex, int32_t endIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SelectTextRange(startIndex, endIndex); } \
  NS_IMETHOD OnSearchBegin(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSearchBegin(); } \
  NS_IMETHOD OnSearchComplete(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSearchComplete(); } \
  NS_IMETHOD OnTextEntered(mozilla::dom::Event *aEvent, bool itemWasSelected, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTextEntered(aEvent, itemWasSelected, _retval); } \
  NS_IMETHOD OnTextReverted(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTextReverted(_retval); } \
  NS_IMETHOD GetConsumeRollupEvent(bool *aConsumeRollupEvent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetConsumeRollupEvent(aConsumeRollupEvent); } \
  NS_IMETHOD GetInPrivateContext(bool *aInPrivateContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInPrivateContext(aInPrivateContext); } \
  NS_IMETHOD GetNoRollupOnCaretMove(bool *aNoRollupOnCaretMove) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNoRollupOnCaretMove(aNoRollupOnCaretMove); } \
  NS_IMETHOD GetNoRollupOnEmptySearch(bool *aNoRollupOnEmptySearch) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNoRollupOnEmptySearch(aNoRollupOnEmptySearch); } \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUserContextId(aUserContextId); } 


#endif /* __gen_nsIAutoCompleteInput_h__ */
