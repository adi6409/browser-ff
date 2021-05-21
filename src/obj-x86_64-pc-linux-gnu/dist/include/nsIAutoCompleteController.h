/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteController.idl
 */

#ifndef __gen_nsIAutoCompleteController_h__
#define __gen_nsIAutoCompleteController_h__


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
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIAutoCompleteController */
#define NS_IAUTOCOMPLETECONTROLLER_IID_STR "ff9f8465-204a-47a6-b3c9-0628b3856684"

#define NS_IAUTOCOMPLETECONTROLLER_IID \
  {0xff9f8465, 0x204a, 0x47a6, \
    { 0xb3, 0xc9, 0x06, 0x28, 0xb3, 0x85, 0x66, 0x84 }}

class NS_NO_VTABLE nsIAutoCompleteController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETECONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteController;

  enum {
    STATUS_NONE = 1U,
    STATUS_SEARCHING = 2U,
    STATUS_COMPLETE_NO_MATCH = 3U,
    STATUS_COMPLETE_MATCH = 4U
  };

  /* [setter_can_run_script] attribute nsIAutoCompleteInput input; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) = 0;
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInput(nsIAutoCompleteInput *aInput) = 0;

  /* readonly attribute unsigned short searchStatus; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchStatus(uint16_t *aSearchStatus) = 0;

  /* readonly attribute unsigned long matchCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchCount(uint32_t *aMatchCount) = 0;

  /* [can_run_script] void startSearch (in AString searchString); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& searchString) = 0;

  /* [can_run_script] void stopSearch (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) = 0;

  /* [can_run_script] boolean handleText (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleText(bool *_retval) = 0;

  /* [can_run_script] boolean handleEnter (in boolean aIsPopupSelection, [optional] in Event aEvent); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEnter(bool aIsPopupSelection, mozilla::dom::Event *aEvent, bool *_retval) = 0;

  /* [can_run_script] boolean handleEscape (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEscape(bool *_retval) = 0;

  /* [can_run_script] void handleStartComposition (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleStartComposition(void) = 0;

  /* void handleEndComposition (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEndComposition(void) = 0;

  /* [can_run_script] void handleTab (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleTab(void) = 0;

  /* [can_run_script] boolean handleKeyNavigation (in unsigned long key); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleKeyNavigation(uint32_t key, bool *_retval) = 0;

  /* [can_run_script] boolean handleDelete (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleDelete(bool *_retval) = 0;

  /* AString getValueAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValueAt(int32_t index, nsAString& _retval) = 0;

  /* AString getLabelAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLabelAt(int32_t index, nsAString& _retval) = 0;

  /* AString getCommentAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCommentAt(int32_t index, nsAString& _retval) = 0;

  /* AString getStyleAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStyleAt(int32_t index, nsAString& _retval) = 0;

  /* AString getImageAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetImageAt(int32_t index, nsAString& _retval) = 0;

  /* AString getFinalCompleteValueAt (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFinalCompleteValueAt(int32_t index, nsAString& _retval) = 0;

  /* attribute AString searchString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchString(nsAString& aSearchString) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchString(const nsAString& aSearchString) = 0;

  /* void setInitiallySelectedIndex (in long index); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetInitiallySelectedIndex(int32_t index) = 0;

  /* [can_run_script] void resetInternalState (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetInternalState(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteController, NS_IAUTOCOMPLETECONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETECONTROLLER \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInput(nsIAutoCompleteInput *aInput) override; \
  NS_IMETHOD GetSearchStatus(uint16_t *aSearchStatus) override; \
  NS_IMETHOD GetMatchCount(uint32_t *aMatchCount) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& searchString) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleText(bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEnter(bool aIsPopupSelection, mozilla::dom::Event *aEvent, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEscape(bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleStartComposition(void) override; \
  NS_IMETHOD HandleEndComposition(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleTab(void) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleKeyNavigation(uint32_t key, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleDelete(bool *_retval) override; \
  NS_IMETHOD GetValueAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetLabelAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetCommentAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetStyleAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetImageAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetFinalCompleteValueAt(int32_t index, nsAString& _retval) override; \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override; \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override; \
  NS_IMETHOD SetInitiallySelectedIndex(int32_t index) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetInternalState(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETECONTROLLER \
  nsresult GetInput(nsIAutoCompleteInput **aInput); \
  MOZ_CAN_RUN_SCRIPT nsresult SetInput(nsIAutoCompleteInput *aInput); \
  nsresult GetSearchStatus(uint16_t *aSearchStatus); \
  nsresult GetMatchCount(uint32_t *aMatchCount); \
  MOZ_CAN_RUN_SCRIPT nsresult StartSearch(const nsAString& searchString); \
  MOZ_CAN_RUN_SCRIPT nsresult StopSearch(void); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleText(bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleEnter(bool aIsPopupSelection, mozilla::dom::Event *aEvent, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleEscape(bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleStartComposition(void); \
  nsresult HandleEndComposition(void); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleTab(void); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleKeyNavigation(uint32_t key, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult HandleDelete(bool *_retval); \
  nsresult GetValueAt(int32_t index, nsAString& _retval); \
  nsresult GetLabelAt(int32_t index, nsAString& _retval); \
  nsresult GetCommentAt(int32_t index, nsAString& _retval); \
  nsresult GetStyleAt(int32_t index, nsAString& _retval); \
  nsresult GetImageAt(int32_t index, nsAString& _retval); \
  nsresult GetFinalCompleteValueAt(int32_t index, nsAString& _retval); \
  nsresult GetSearchString(nsAString& aSearchString); \
  nsresult SetSearchString(const nsAString& aSearchString); \
  nsresult SetInitiallySelectedIndex(int32_t index); \
  MOZ_CAN_RUN_SCRIPT nsresult ResetInternalState(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETECONTROLLER(_to) \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override { return _to GetInput(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInput(nsIAutoCompleteInput *aInput) override { return _to SetInput(aInput); } \
  NS_IMETHOD GetSearchStatus(uint16_t *aSearchStatus) override { return _to GetSearchStatus(aSearchStatus); } \
  NS_IMETHOD GetMatchCount(uint32_t *aMatchCount) override { return _to GetMatchCount(aMatchCount); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& searchString) override { return _to StartSearch(searchString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) override { return _to StopSearch(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleText(bool *_retval) override { return _to HandleText(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEnter(bool aIsPopupSelection, mozilla::dom::Event *aEvent, bool *_retval) override { return _to HandleEnter(aIsPopupSelection, aEvent, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEscape(bool *_retval) override { return _to HandleEscape(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleStartComposition(void) override { return _to HandleStartComposition(); } \
  NS_IMETHOD HandleEndComposition(void) override { return _to HandleEndComposition(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleTab(void) override { return _to HandleTab(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleKeyNavigation(uint32_t key, bool *_retval) override { return _to HandleKeyNavigation(key, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleDelete(bool *_retval) override { return _to HandleDelete(_retval); } \
  NS_IMETHOD GetValueAt(int32_t index, nsAString& _retval) override { return _to GetValueAt(index, _retval); } \
  NS_IMETHOD GetLabelAt(int32_t index, nsAString& _retval) override { return _to GetLabelAt(index, _retval); } \
  NS_IMETHOD GetCommentAt(int32_t index, nsAString& _retval) override { return _to GetCommentAt(index, _retval); } \
  NS_IMETHOD GetStyleAt(int32_t index, nsAString& _retval) override { return _to GetStyleAt(index, _retval); } \
  NS_IMETHOD GetImageAt(int32_t index, nsAString& _retval) override { return _to GetImageAt(index, _retval); } \
  NS_IMETHOD GetFinalCompleteValueAt(int32_t index, nsAString& _retval) override { return _to GetFinalCompleteValueAt(index, _retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return _to GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return _to SetSearchString(aSearchString); } \
  NS_IMETHOD SetInitiallySelectedIndex(int32_t index) override { return _to SetInitiallySelectedIndex(index); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetInternalState(void) override { return _to ResetInternalState(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETECONTROLLER(_to) \
  NS_IMETHOD GetInput(nsIAutoCompleteInput **aInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInput(aInput); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SetInput(nsIAutoCompleteInput *aInput) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInput(aInput); } \
  NS_IMETHOD GetSearchStatus(uint16_t *aSearchStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchStatus(aSearchStatus); } \
  NS_IMETHOD GetMatchCount(uint32_t *aMatchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchCount(aMatchCount); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& searchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartSearch(searchString); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopSearch(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleText(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleText(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEnter(bool aIsPopupSelection, mozilla::dom::Event *aEvent, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEnter(aIsPopupSelection, aEvent, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEscape(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEscape(_retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleStartComposition(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleStartComposition(); } \
  NS_IMETHOD HandleEndComposition(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEndComposition(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleTab(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleTab(); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleKeyNavigation(uint32_t key, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleKeyNavigation(key, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleDelete(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleDelete(_retval); } \
  NS_IMETHOD GetValueAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValueAt(index, _retval); } \
  NS_IMETHOD GetLabelAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLabelAt(index, _retval); } \
  NS_IMETHOD GetCommentAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCommentAt(index, _retval); } \
  NS_IMETHOD GetStyleAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStyleAt(index, _retval); } \
  NS_IMETHOD GetImageAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetImageAt(index, _retval); } \
  NS_IMETHOD GetFinalCompleteValueAt(int32_t index, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFinalCompleteValueAt(index, _retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchString(aSearchString); } \
  NS_IMETHOD SetInitiallySelectedIndex(int32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInitiallySelectedIndex(index); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ResetInternalState(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetInternalState(); } 


#endif /* __gen_nsIAutoCompleteController_h__ */
