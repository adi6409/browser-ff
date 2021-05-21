/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/spellchecker/nsIInlineSpellChecker.idl
 */

#ifndef __gen_nsIInlineSpellChecker_h__
#define __gen_nsIInlineSpellChecker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEditor; /* forward declaration */

class nsIEditorSpellCheck; /* forward declaration */

class nsINode; /* webidl Node */

class nsRange; /* webidl Range */


/* starting interface:    nsIInlineSpellChecker */
#define NS_IINLINESPELLCHECKER_IID_STR "b7b7a77c-40c4-4196-b0b7-b0338243b3fe"

#define NS_IINLINESPELLCHECKER_IID \
  {0xb7b7a77c, 0x40c4, 0x4196, \
    { 0xb0, 0xb7, 0xb0, 0x33, 0x82, 0x43, 0xb3, 0xfe }}

class NS_NO_VTABLE nsIInlineSpellChecker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINLINESPELLCHECKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInlineSpellChecker;

  /* readonly attribute nsIEditorSpellCheck spellChecker; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSpellChecker(nsIEditorSpellCheck **aSpellChecker) = 0;

  /* void init (in nsIEditor aEditor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIEditor *aEditor) = 0;

  /* void cleanup (in boolean aDestroyingFrames); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cleanup(bool aDestroyingFrames) = 0;

  /* attribute boolean enableRealTimeSpell; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnableRealTimeSpell(bool *aEnableRealTimeSpell) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEnableRealTimeSpell(bool aEnableRealTimeSpell) = 0;

  /* void spellCheckRange (in Range aSelection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpellCheckRange(nsRange *aSelection) = 0;

  /* Range getMisspelledWord (in Node aNode, in long aOffset); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMisspelledWord(nsINode *aNode, int32_t aOffset, nsRange **_retval) = 0;

  /* [can_run_script] void replaceWord (in Node aNode, in long aOffset, in AString aNewword); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(nsINode *aNode, int32_t aOffset, const nsAString& aNewword) = 0;

  /* void addWordToDictionary (in AString aWord); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddWordToDictionary(const nsAString& aWord) = 0;

  /* void removeWordFromDictionary (in AString aWord); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveWordFromDictionary(const nsAString& aWord) = 0;

  /* void ignoreWord (in AString aWord); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IgnoreWord(const nsAString& aWord) = 0;

  /* void ignoreWords (in Array<AString> aWordsToIgnore); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IgnoreWords(const nsTArray<nsString >& aWordsToIgnore) = 0;

  /* void updateCurrentDictionary (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateCurrentDictionary(void) = 0;

  /* readonly attribute boolean spellCheckPending; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSpellCheckPending(bool *aSpellCheckPending) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInlineSpellChecker, NS_IINLINESPELLCHECKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINLINESPELLCHECKER \
  NS_IMETHOD GetSpellChecker(nsIEditorSpellCheck **aSpellChecker) override; \
  NS_IMETHOD Init(nsIEditor *aEditor) override; \
  NS_IMETHOD Cleanup(bool aDestroyingFrames) override; \
  NS_IMETHOD GetEnableRealTimeSpell(bool *aEnableRealTimeSpell) override; \
  NS_IMETHOD SetEnableRealTimeSpell(bool aEnableRealTimeSpell) override; \
  NS_IMETHOD SpellCheckRange(nsRange *aSelection) override; \
  NS_IMETHOD GetMisspelledWord(nsINode *aNode, int32_t aOffset, nsRange **_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(nsINode *aNode, int32_t aOffset, const nsAString& aNewword) override; \
  NS_IMETHOD AddWordToDictionary(const nsAString& aWord) override; \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& aWord) override; \
  NS_IMETHOD IgnoreWord(const nsAString& aWord) override; \
  NS_IMETHOD IgnoreWords(const nsTArray<nsString >& aWordsToIgnore) override; \
  NS_IMETHOD UpdateCurrentDictionary(void) override; \
  NS_IMETHOD GetSpellCheckPending(bool *aSpellCheckPending) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINLINESPELLCHECKER \
  nsresult GetSpellChecker(nsIEditorSpellCheck **aSpellChecker); \
  nsresult Init(nsIEditor *aEditor); \
  nsresult Cleanup(bool aDestroyingFrames); \
  nsresult GetEnableRealTimeSpell(bool *aEnableRealTimeSpell); \
  nsresult SetEnableRealTimeSpell(bool aEnableRealTimeSpell); \
  nsresult SpellCheckRange(nsRange *aSelection); \
  nsresult GetMisspelledWord(nsINode *aNode, int32_t aOffset, nsRange **_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult ReplaceWord(nsINode *aNode, int32_t aOffset, const nsAString& aNewword); \
  nsresult AddWordToDictionary(const nsAString& aWord); \
  nsresult RemoveWordFromDictionary(const nsAString& aWord); \
  nsresult IgnoreWord(const nsAString& aWord); \
  nsresult IgnoreWords(const nsTArray<nsString >& aWordsToIgnore); \
  nsresult UpdateCurrentDictionary(void); \
  nsresult GetSpellCheckPending(bool *aSpellCheckPending); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINLINESPELLCHECKER(_to) \
  NS_IMETHOD GetSpellChecker(nsIEditorSpellCheck **aSpellChecker) override { return _to GetSpellChecker(aSpellChecker); } \
  NS_IMETHOD Init(nsIEditor *aEditor) override { return _to Init(aEditor); } \
  NS_IMETHOD Cleanup(bool aDestroyingFrames) override { return _to Cleanup(aDestroyingFrames); } \
  NS_IMETHOD GetEnableRealTimeSpell(bool *aEnableRealTimeSpell) override { return _to GetEnableRealTimeSpell(aEnableRealTimeSpell); } \
  NS_IMETHOD SetEnableRealTimeSpell(bool aEnableRealTimeSpell) override { return _to SetEnableRealTimeSpell(aEnableRealTimeSpell); } \
  NS_IMETHOD SpellCheckRange(nsRange *aSelection) override { return _to SpellCheckRange(aSelection); } \
  NS_IMETHOD GetMisspelledWord(nsINode *aNode, int32_t aOffset, nsRange **_retval) override { return _to GetMisspelledWord(aNode, aOffset, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(nsINode *aNode, int32_t aOffset, const nsAString& aNewword) override { return _to ReplaceWord(aNode, aOffset, aNewword); } \
  NS_IMETHOD AddWordToDictionary(const nsAString& aWord) override { return _to AddWordToDictionary(aWord); } \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& aWord) override { return _to RemoveWordFromDictionary(aWord); } \
  NS_IMETHOD IgnoreWord(const nsAString& aWord) override { return _to IgnoreWord(aWord); } \
  NS_IMETHOD IgnoreWords(const nsTArray<nsString >& aWordsToIgnore) override { return _to IgnoreWords(aWordsToIgnore); } \
  NS_IMETHOD UpdateCurrentDictionary(void) override { return _to UpdateCurrentDictionary(); } \
  NS_IMETHOD GetSpellCheckPending(bool *aSpellCheckPending) override { return _to GetSpellCheckPending(aSpellCheckPending); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINLINESPELLCHECKER(_to) \
  NS_IMETHOD GetSpellChecker(nsIEditorSpellCheck **aSpellChecker) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpellChecker(aSpellChecker); } \
  NS_IMETHOD Init(nsIEditor *aEditor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aEditor); } \
  NS_IMETHOD Cleanup(bool aDestroyingFrames) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cleanup(aDestroyingFrames); } \
  NS_IMETHOD GetEnableRealTimeSpell(bool *aEnableRealTimeSpell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnableRealTimeSpell(aEnableRealTimeSpell); } \
  NS_IMETHOD SetEnableRealTimeSpell(bool aEnableRealTimeSpell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEnableRealTimeSpell(aEnableRealTimeSpell); } \
  NS_IMETHOD SpellCheckRange(nsRange *aSelection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpellCheckRange(aSelection); } \
  NS_IMETHOD GetMisspelledWord(nsINode *aNode, int32_t aOffset, nsRange **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMisspelledWord(aNode, aOffset, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(nsINode *aNode, int32_t aOffset, const nsAString& aNewword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceWord(aNode, aOffset, aNewword); } \
  NS_IMETHOD AddWordToDictionary(const nsAString& aWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWordToDictionary(aWord); } \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& aWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWordFromDictionary(aWord); } \
  NS_IMETHOD IgnoreWord(const nsAString& aWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IgnoreWord(aWord); } \
  NS_IMETHOD IgnoreWords(const nsTArray<nsString >& aWordsToIgnore) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IgnoreWords(aWordsToIgnore); } \
  NS_IMETHOD UpdateCurrentDictionary(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateCurrentDictionary(); } \
  NS_IMETHOD GetSpellCheckPending(bool *aSpellCheckPending) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpellCheckPending(aSpellCheckPending); } 


#endif /* __gen_nsIInlineSpellChecker_h__ */
