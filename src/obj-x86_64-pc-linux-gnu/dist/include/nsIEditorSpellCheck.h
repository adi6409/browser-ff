/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorSpellCheck.idl
 */

#ifndef __gen_nsIEditorSpellCheck_h__
#define __gen_nsIEditorSpellCheck_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
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

class nsIEditorSpellCheckCallback; /* forward declaration */


/* starting interface:    nsIEditorSpellCheck */
#define NS_IEDITORSPELLCHECK_IID_STR "a171c25f-e4a8-4d08-adef-b797e6377bdc"

#define NS_IEDITORSPELLCHECK_IID \
  {0xa171c25f, 0xe4a8, 0x4d08, \
    { 0xad, 0xef, 0xb7, 0x97, 0xe6, 0x37, 0x7b, 0xdc }}

class NS_NO_VTABLE nsIEditorSpellCheck : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITORSPELLCHECK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditorSpellCheck;

  /* boolean canSpellCheck (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanSpellCheck(bool *_retval) = 0;

  /* void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitSpellChecker(nsIEditor *editor, bool enableSelectionChecking, nsIEditorSpellCheckCallback *callback) = 0;

  /* [can_run_script] AString GetNextMisspelledWord (); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextMisspelledWord(nsAString& _retval) = 0;

  /* AString GetSuggestedWord (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSuggestedWord(nsAString& _retval) = 0;

  /* boolean CheckCurrentWord (in AString suggestedWord); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CheckCurrentWord(const nsAString& suggestedWord, bool *_retval) = 0;

  /* [can_run_script] void ReplaceWord (in AString misspelledWord, in AString replaceWord, in boolean allOccurrences); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(const nsAString& misspelledWord, const nsAString& replaceWord, bool allOccurrences) = 0;

  /* void IgnoreWordAllOccurrences (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IgnoreWordAllOccurrences(const nsAString& word) = 0;

  /* void GetPersonalDictionary (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersonalDictionary(void) = 0;

  /* AString GetPersonalDictionaryWord (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersonalDictionaryWord(nsAString& _retval) = 0;

  /* void AddWordToDictionary (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddWordToDictionary(const nsAString& word) = 0;

  /* void RemoveWordFromDictionary (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveWordFromDictionary(const nsAString& word) = 0;

  /* Array<ACString> GetDictionaryList (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) = 0;

  /* ACString GetCurrentDictionary (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentDictionary(nsACString& _retval) = 0;

  /* void SetCurrentDictionary (in ACString dictionary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentDictionary(const nsACString& dictionary) = 0;

  /* void UninitSpellChecker (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UninitSpellChecker(void) = 0;

  enum {
    FILTERTYPE_NORMAL = 1U,
    FILTERTYPE_MAIL = 2U
  };

  /* void setFilterType (in unsigned long filterType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFilterType(uint32_t filterType) = 0;

  /* void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateCurrentDictionary(nsIEditorSpellCheckCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditorSpellCheck, NS_IEDITORSPELLCHECK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITORSPELLCHECK \
  NS_IMETHOD CanSpellCheck(bool *_retval) override; \
  NS_IMETHOD InitSpellChecker(nsIEditor *editor, bool enableSelectionChecking, nsIEditorSpellCheckCallback *callback) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextMisspelledWord(nsAString& _retval) override; \
  NS_IMETHOD GetSuggestedWord(nsAString& _retval) override; \
  NS_IMETHOD CheckCurrentWord(const nsAString& suggestedWord, bool *_retval) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(const nsAString& misspelledWord, const nsAString& replaceWord, bool allOccurrences) override; \
  NS_IMETHOD IgnoreWordAllOccurrences(const nsAString& word) override; \
  NS_IMETHOD GetPersonalDictionary(void) override; \
  NS_IMETHOD GetPersonalDictionaryWord(nsAString& _retval) override; \
  NS_IMETHOD AddWordToDictionary(const nsAString& word) override; \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& word) override; \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetCurrentDictionary(nsACString& _retval) override; \
  NS_IMETHOD SetCurrentDictionary(const nsACString& dictionary) override; \
  NS_IMETHOD UninitSpellChecker(void) override; \
  NS_IMETHOD SetFilterType(uint32_t filterType) override; \
  NS_IMETHOD UpdateCurrentDictionary(nsIEditorSpellCheckCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITORSPELLCHECK \
  nsresult CanSpellCheck(bool *_retval); \
  nsresult InitSpellChecker(nsIEditor *editor, bool enableSelectionChecking, nsIEditorSpellCheckCallback *callback); \
  MOZ_CAN_RUN_SCRIPT nsresult GetNextMisspelledWord(nsAString& _retval); \
  nsresult GetSuggestedWord(nsAString& _retval); \
  nsresult CheckCurrentWord(const nsAString& suggestedWord, bool *_retval); \
  MOZ_CAN_RUN_SCRIPT nsresult ReplaceWord(const nsAString& misspelledWord, const nsAString& replaceWord, bool allOccurrences); \
  nsresult IgnoreWordAllOccurrences(const nsAString& word); \
  nsresult GetPersonalDictionary(void); \
  nsresult GetPersonalDictionaryWord(nsAString& _retval); \
  nsresult AddWordToDictionary(const nsAString& word); \
  nsresult RemoveWordFromDictionary(const nsAString& word); \
  nsresult GetDictionaryList(nsTArray<nsCString >& _retval); \
  nsresult GetCurrentDictionary(nsACString& _retval); \
  nsresult SetCurrentDictionary(const nsACString& dictionary); \
  nsresult UninitSpellChecker(void); \
  nsresult SetFilterType(uint32_t filterType); \
  nsresult UpdateCurrentDictionary(nsIEditorSpellCheckCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITORSPELLCHECK(_to) \
  NS_IMETHOD CanSpellCheck(bool *_retval) override { return _to CanSpellCheck(_retval); } \
  NS_IMETHOD InitSpellChecker(nsIEditor *editor, bool enableSelectionChecking, nsIEditorSpellCheckCallback *callback) override { return _to InitSpellChecker(editor, enableSelectionChecking, callback); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextMisspelledWord(nsAString& _retval) override { return _to GetNextMisspelledWord(_retval); } \
  NS_IMETHOD GetSuggestedWord(nsAString& _retval) override { return _to GetSuggestedWord(_retval); } \
  NS_IMETHOD CheckCurrentWord(const nsAString& suggestedWord, bool *_retval) override { return _to CheckCurrentWord(suggestedWord, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(const nsAString& misspelledWord, const nsAString& replaceWord, bool allOccurrences) override { return _to ReplaceWord(misspelledWord, replaceWord, allOccurrences); } \
  NS_IMETHOD IgnoreWordAllOccurrences(const nsAString& word) override { return _to IgnoreWordAllOccurrences(word); } \
  NS_IMETHOD GetPersonalDictionary(void) override { return _to GetPersonalDictionary(); } \
  NS_IMETHOD GetPersonalDictionaryWord(nsAString& _retval) override { return _to GetPersonalDictionaryWord(_retval); } \
  NS_IMETHOD AddWordToDictionary(const nsAString& word) override { return _to AddWordToDictionary(word); } \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& word) override { return _to RemoveWordFromDictionary(word); } \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override { return _to GetDictionaryList(_retval); } \
  NS_IMETHOD GetCurrentDictionary(nsACString& _retval) override { return _to GetCurrentDictionary(_retval); } \
  NS_IMETHOD SetCurrentDictionary(const nsACString& dictionary) override { return _to SetCurrentDictionary(dictionary); } \
  NS_IMETHOD UninitSpellChecker(void) override { return _to UninitSpellChecker(); } \
  NS_IMETHOD SetFilterType(uint32_t filterType) override { return _to SetFilterType(filterType); } \
  NS_IMETHOD UpdateCurrentDictionary(nsIEditorSpellCheckCallback *callback) override { return _to UpdateCurrentDictionary(callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITORSPELLCHECK(_to) \
  NS_IMETHOD CanSpellCheck(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanSpellCheck(_retval); } \
  NS_IMETHOD InitSpellChecker(nsIEditor *editor, bool enableSelectionChecking, nsIEditorSpellCheckCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitSpellChecker(editor, enableSelectionChecking, callback); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextMisspelledWord(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextMisspelledWord(_retval); } \
  NS_IMETHOD GetSuggestedWord(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuggestedWord(_retval); } \
  NS_IMETHOD CheckCurrentWord(const nsAString& suggestedWord, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckCurrentWord(suggestedWord, _retval); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD ReplaceWord(const nsAString& misspelledWord, const nsAString& replaceWord, bool allOccurrences) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReplaceWord(misspelledWord, replaceWord, allOccurrences); } \
  NS_IMETHOD IgnoreWordAllOccurrences(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IgnoreWordAllOccurrences(word); } \
  NS_IMETHOD GetPersonalDictionary(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersonalDictionary(); } \
  NS_IMETHOD GetPersonalDictionaryWord(nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersonalDictionaryWord(_retval); } \
  NS_IMETHOD AddWordToDictionary(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWordToDictionary(word); } \
  NS_IMETHOD RemoveWordFromDictionary(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWordFromDictionary(word); } \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDictionaryList(_retval); } \
  NS_IMETHOD GetCurrentDictionary(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentDictionary(_retval); } \
  NS_IMETHOD SetCurrentDictionary(const nsACString& dictionary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentDictionary(dictionary); } \
  NS_IMETHOD UninitSpellChecker(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UninitSpellChecker(); } \
  NS_IMETHOD SetFilterType(uint32_t filterType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFilterType(filterType); } \
  NS_IMETHOD UpdateCurrentDictionary(nsIEditorSpellCheckCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateCurrentDictionary(callback); } 


/* starting interface:    nsIEditorSpellCheckCallback */
#define NS_IEDITORSPELLCHECKCALLBACK_IID_STR "5f0a4bab-8538-4074-89d3-2f0e866a1c0b"

#define NS_IEDITORSPELLCHECKCALLBACK_IID \
  {0x5f0a4bab, 0x8538, 0x4074, \
    { 0x89, 0xd3, 0x2f, 0x0e, 0x86, 0x6a, 0x1c, 0x0b }}

class NS_NO_VTABLE nsIEditorSpellCheckCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEDITORSPELLCHECKCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEditorSpellCheckCallback;

  /* void editorSpellCheckDone (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EditorSpellCheckDone(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEditorSpellCheckCallback, NS_IEDITORSPELLCHECKCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEDITORSPELLCHECKCALLBACK \
  NS_IMETHOD EditorSpellCheckDone(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEDITORSPELLCHECKCALLBACK \
  nsresult EditorSpellCheckDone(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEDITORSPELLCHECKCALLBACK(_to) \
  NS_IMETHOD EditorSpellCheckDone(void) override { return _to EditorSpellCheckDone(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEDITORSPELLCHECKCALLBACK(_to) \
  NS_IMETHOD EditorSpellCheckDone(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EditorSpellCheckDone(); } 


#endif /* __gen_nsIEditorSpellCheck_h__ */
