/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsITextInputProcessor.idl
 */

#ifndef __gen_nsITextInputProcessor_h__
#define __gen_nsITextInputProcessor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

class nsITextInputProcessorCallback; /* forward declaration */

namespace mozilla {
namespace dom {
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsITextInputProcessor */
#define NS_ITEXTINPUTPROCESSOR_IID_STR "47ae2181-2e98-4d58-84a2-b8db6764ce9a"

#define NS_ITEXTINPUTPROCESSOR_IID \
  {0x47ae2181, 0x2e98, 0x4d58, \
    { 0x84, 0xa2, 0xb8, 0xdb, 0x67, 0x64, 0xce, 0x9a }}

class NS_NO_VTABLE nsITextInputProcessor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITEXTINPUTPROCESSOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITextInputProcessor;

  /* readonly attribute boolean hasComposition; */
  NS_IMETHOD GetHasComposition(bool *aHasComposition) = 0;

  /* boolean beginInputTransaction (in mozIDOMWindow aWindow, in nsITextInputProcessorCallback aCallback); */
  NS_IMETHOD BeginInputTransaction(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, bool *_retval) = 0;

  /* [optional_argc] boolean beginInputTransactionForTests (in mozIDOMWindow aWindow, [optional] in nsITextInputProcessorCallback aCallback); */
  NS_IMETHOD BeginInputTransactionForTests(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, uint8_t _argc, bool *_retval) = 0;

  /* [can_run_script,optional_argc] boolean startComposition ([optional] in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) = 0;

  /* void setPendingCompositionString (in AString aString); */
  NS_IMETHOD SetPendingCompositionString(const nsAString& aString) = 0;

  enum {
    ATTR_RAW_CLAUSE = 2U,
    ATTR_SELECTED_RAW_CLAUSE = 3U,
    ATTR_CONVERTED_CLAUSE = 4U,
    ATTR_SELECTED_CLAUSE = 5U
  };

  /* void appendClauseToPendingComposition (in unsigned long aLength, in unsigned long aAttribute); */
  NS_IMETHOD AppendClauseToPendingComposition(uint32_t aLength, uint32_t aAttribute) = 0;

  /* void setCaretInPendingComposition (in unsigned long aOffset); */
  NS_IMETHOD SetCaretInPendingComposition(uint32_t aOffset) = 0;

  /* [can_run_script,optional_argc] boolean flushPendingComposition ([optional] in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FlushPendingComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) = 0;

  /* [can_run_script,optional_argc] void commitComposition ([optional] in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) = 0;

  /* [can_run_script,optional_argc] boolean commitCompositionWith (in AString aCommitString, [optional] in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitCompositionWith(const nsAString& aCommitString, mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) = 0;

  /* [can_run_script,optional_argc] void cancelComposition ([optional] in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CancelComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) = 0;

  enum {
    KEY_DEFAULT_PREVENTED = 1U,
    KEY_NON_PRINTABLE_KEY = 2U,
    KEY_FORCE_PRINTABLE_KEY = 4U,
    KEY_KEEP_KEY_LOCATION_STANDARD = 8U,
    KEY_KEEP_KEYCODE_ZERO = 16U,
    KEY_DONT_DISPATCH_MODIFIER_KEY_EVENT = 32U,
    KEY_DONT_MARK_KEYDOWN_AS_PROCESSED = 64U,
    KEY_MARK_KEYUP_AS_PROCESSED = 128U,
    KEYEVENT_NOT_CONSUMED = 0U,
    KEYDOWN_IS_CONSUMED = 1U,
    KEYPRESS_IS_CONSUMED = 2U
  };

  /* [can_run_script,optional_argc] unsigned long keydown (in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Keydown(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, uint32_t *_retval) = 0;

  /* [optional_argc] boolean keyup (in Event aKeyboardEvent, [optional] in unsigned long aKeyFlags); */
  NS_IMETHOD Keyup(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) = 0;

  /* boolean getModifierState (in AString aModifierKey); */
  NS_IMETHOD GetModifierState(const nsAString& aModifierKey, bool *_retval) = 0;

  /* void shareModifierStateOf (in nsITextInputProcessor aOther); */
  NS_IMETHOD ShareModifierStateOf(nsITextInputProcessor *aOther) = 0;

  /* [optional_argc] AString computeCodeValueOfNonPrintableKey (in AString aKeyValue, [optional] in jsval aLocation); */
  NS_IMETHOD ComputeCodeValueOfNonPrintableKey(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) = 0;

  /* [optional_argc] AString guessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout (in AString aKeyValue, [optional] in jsval aLocation); */
  NS_IMETHOD GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) = 0;

  /* [optional_argc] unsigned long guessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout (in AString aKeyValue, [optional] in jsval aLocation); */
  NS_IMETHOD GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, uint32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITextInputProcessor, NS_ITEXTINPUTPROCESSOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITEXTINPUTPROCESSOR \
  NS_IMETHOD GetHasComposition(bool *aHasComposition) override; \
  NS_IMETHOD BeginInputTransaction(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, bool *_retval) override; \
  NS_IMETHOD BeginInputTransactionForTests(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, uint8_t _argc, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD SetPendingCompositionString(const nsAString& aString) override; \
  NS_IMETHOD AppendClauseToPendingComposition(uint32_t aLength, uint32_t aAttribute) override; \
  NS_IMETHOD SetCaretInPendingComposition(uint32_t aOffset) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FlushPendingComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitCompositionWith(const nsAString& aCommitString, mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CancelComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Keydown(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, uint32_t *_retval) override; \
  NS_IMETHOD Keyup(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override; \
  NS_IMETHOD GetModifierState(const nsAString& aModifierKey, bool *_retval) override; \
  NS_IMETHOD ShareModifierStateOf(nsITextInputProcessor *aOther) override; \
  NS_IMETHOD ComputeCodeValueOfNonPrintableKey(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override; \
  NS_IMETHOD GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override; \
  NS_IMETHOD GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, uint32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITEXTINPUTPROCESSOR \
  nsresult GetHasComposition(bool *aHasComposition); \
  nsresult BeginInputTransaction(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, bool *_retval); \
  nsresult BeginInputTransactionForTests(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, uint8_t _argc, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult StartComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval); \
  nsresult SetPendingCompositionString(const nsAString& aString); \
  nsresult AppendClauseToPendingComposition(uint32_t aLength, uint32_t aAttribute); \
  nsresult SetCaretInPendingComposition(uint32_t aOffset); \
  MOZ_CAN_RUN_SCRIPT nsresult FlushPendingComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult CommitComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc); \
  MOZ_CAN_RUN_SCRIPT nsresult CommitCompositionWith(const nsAString& aCommitString, mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult CancelComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc); \
  MOZ_CAN_RUN_SCRIPT nsresult Keydown(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, uint32_t *_retval); \
  nsresult Keyup(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval); \
  nsresult GetModifierState(const nsAString& aModifierKey, bool *_retval); \
  nsresult ShareModifierStateOf(nsITextInputProcessor *aOther); \
  nsresult ComputeCodeValueOfNonPrintableKey(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval); \
  nsresult GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval); \
  nsresult GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, uint32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITEXTINPUTPROCESSOR(_to) \
  NS_IMETHOD GetHasComposition(bool *aHasComposition) override { return _to GetHasComposition(aHasComposition); } \
  NS_IMETHOD BeginInputTransaction(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, bool *_retval) override { return _to BeginInputTransaction(aWindow, aCallback, _retval); } \
  NS_IMETHOD BeginInputTransactionForTests(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, uint8_t _argc, bool *_retval) override { return _to BeginInputTransactionForTests(aWindow, aCallback, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return _to StartComposition(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD SetPendingCompositionString(const nsAString& aString) override { return _to SetPendingCompositionString(aString); } \
  NS_IMETHOD AppendClauseToPendingComposition(uint32_t aLength, uint32_t aAttribute) override { return _to AppendClauseToPendingComposition(aLength, aAttribute); } \
  NS_IMETHOD SetCaretInPendingComposition(uint32_t aOffset) override { return _to SetCaretInPendingComposition(aOffset); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FlushPendingComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return _to FlushPendingComposition(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override { return _to CommitComposition(aKeyboardEvent, aKeyFlags, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitCompositionWith(const nsAString& aCommitString, mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return _to CommitCompositionWith(aCommitString, aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CancelComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override { return _to CancelComposition(aKeyboardEvent, aKeyFlags, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Keydown(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, uint32_t *_retval) override { return _to Keydown(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD Keyup(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return _to Keyup(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD GetModifierState(const nsAString& aModifierKey, bool *_retval) override { return _to GetModifierState(aModifierKey, _retval); } \
  NS_IMETHOD ShareModifierStateOf(nsITextInputProcessor *aOther) override { return _to ShareModifierStateOf(aOther); } \
  NS_IMETHOD ComputeCodeValueOfNonPrintableKey(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override { return _to ComputeCodeValueOfNonPrintableKey(aKeyValue, aLocation, _argc, _retval); } \
  NS_IMETHOD GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override { return _to GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(aKeyValue, aLocation, _argc, _retval); } \
  NS_IMETHOD GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, uint32_t *_retval) override { return _to GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(aKeyValue, aLocation, _argc, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITEXTINPUTPROCESSOR(_to) \
  NS_IMETHOD GetHasComposition(bool *aHasComposition) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasComposition(aHasComposition); } \
  NS_IMETHOD BeginInputTransaction(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginInputTransaction(aWindow, aCallback, _retval); } \
  NS_IMETHOD BeginInputTransactionForTests(mozIDOMWindow *aWindow, nsITextInputProcessorCallback *aCallback, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginInputTransactionForTests(aWindow, aCallback, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD StartComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartComposition(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD SetPendingCompositionString(const nsAString& aString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPendingCompositionString(aString); } \
  NS_IMETHOD AppendClauseToPendingComposition(uint32_t aLength, uint32_t aAttribute) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendClauseToPendingComposition(aLength, aAttribute); } \
  NS_IMETHOD SetCaretInPendingComposition(uint32_t aOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaretInPendingComposition(aOffset); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD FlushPendingComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FlushPendingComposition(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CommitComposition(aKeyboardEvent, aKeyFlags, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CommitCompositionWith(const nsAString& aCommitString, mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CommitCompositionWith(aCommitString, aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD CancelComposition(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelComposition(aKeyboardEvent, aKeyFlags, _argc); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD Keydown(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Keydown(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD Keyup(mozilla::dom::Event *aKeyboardEvent, uint32_t aKeyFlags, uint8_t _argc, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Keyup(aKeyboardEvent, aKeyFlags, _argc, _retval); } \
  NS_IMETHOD GetModifierState(const nsAString& aModifierKey, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModifierState(aModifierKey, _retval); } \
  NS_IMETHOD ShareModifierStateOf(nsITextInputProcessor *aOther) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShareModifierStateOf(aOther); } \
  NS_IMETHOD ComputeCodeValueOfNonPrintableKey(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ComputeCodeValueOfNonPrintableKey(aKeyValue, aLocation, _argc, _retval); } \
  NS_IMETHOD GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GuessCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(aKeyValue, aLocation, _argc, _retval); } \
  NS_IMETHOD GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(const nsAString& aKeyValue, JS::HandleValue aLocation, uint8_t _argc, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GuessKeyCodeValueOfPrintableKeyInUSEnglishKeyboardLayout(aKeyValue, aLocation, _argc, _retval); } 

#define TEXT_INPUT_PROCESSOR_CID \
  { 0xcaaab47f, 0x1e31, 0x478e, \
    { 0x89, 0x19, 0x97, 0x09, 0x04, 0xe9, 0xcb, 0x72 } }
#define TEXT_INPUT_PROCESSOR_CONTRACTID \
  "@mozilla.org/text-input-processor;1"

#endif /* __gen_nsITextInputProcessor_h__ */
