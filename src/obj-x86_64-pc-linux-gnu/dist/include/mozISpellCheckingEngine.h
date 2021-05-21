/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozISpellCheckingEngine.idl
 */

#ifndef __gen_mozISpellCheckingEngine_h__
#define __gen_mozISpellCheckingEngine_h__


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
class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */

class mozIPersonalDictionary; /* forward declaration */


/* starting interface:    mozISpellCheckingEngine */
#define MOZISPELLCHECKINGENGINE_IID_STR "8ba643a4-7ddc-4662-b976-7ec123843f10"

#define MOZISPELLCHECKINGENGINE_IID \
  {0x8ba643a4, 0x7ddc, 0x4662, \
    { 0xb9, 0x76, 0x7e, 0xc1, 0x23, 0x84, 0x3f, 0x10 }}

class NS_NO_VTABLE mozISpellCheckingEngine : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISPELLCHECKINGENGINE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozISpellCheckingEngine;

  /* attribute ACString dictionary; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDictionary(nsACString& aDictionary) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDictionary(const nsACString& aDictionary) = 0;

  /* attribute mozIPersonalDictionary personalDictionary; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersonalDictionary(mozIPersonalDictionary **aPersonalDictionary) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPersonalDictionary(mozIPersonalDictionary *aPersonalDictionary) = 0;

  /* Array<ACString> getDictionaryList (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) = 0;

  /* boolean check (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Check(const nsAString& word, bool *_retval) = 0;

  /* Array<AString> suggest (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Suggest(const nsAString& word, nsTArray<nsString >& _retval) = 0;

  /* void loadDictionariesFromDir (in nsIFile dir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD LoadDictionariesFromDir(nsIFile *dir) = 0;

  /* void addDirectory (in nsIFile dir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDirectory(nsIFile *dir) = 0;

  /* void removeDirectory (in nsIFile dir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDirectory(nsIFile *dir) = 0;

  /* void addDictionary (in AString lang, in nsIURI file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDictionary(const nsAString& lang, nsIURI *file) = 0;

  /* bool removeDictionary (in AString lang, in nsIURI file); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveDictionary(const nsAString& lang, nsIURI *file, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozISpellCheckingEngine, MOZISPELLCHECKINGENGINE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISPELLCHECKINGENGINE \
  NS_IMETHOD GetDictionary(nsACString& aDictionary) override; \
  NS_IMETHOD SetDictionary(const nsACString& aDictionary) override; \
  NS_IMETHOD GetPersonalDictionary(mozIPersonalDictionary **aPersonalDictionary) override; \
  NS_IMETHOD SetPersonalDictionary(mozIPersonalDictionary *aPersonalDictionary) override; \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override; \
  NS_IMETHOD Suggest(const nsAString& word, nsTArray<nsString >& _retval) override; \
  NS_IMETHOD LoadDictionariesFromDir(nsIFile *dir) override; \
  NS_IMETHOD AddDirectory(nsIFile *dir) override; \
  NS_IMETHOD RemoveDirectory(nsIFile *dir) override; \
  NS_IMETHOD AddDictionary(const nsAString& lang, nsIURI *file) override; \
  NS_IMETHOD RemoveDictionary(const nsAString& lang, nsIURI *file, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISPELLCHECKINGENGINE \
  nsresult GetDictionary(nsACString& aDictionary); \
  nsresult SetDictionary(const nsACString& aDictionary); \
  nsresult GetPersonalDictionary(mozIPersonalDictionary **aPersonalDictionary); \
  nsresult SetPersonalDictionary(mozIPersonalDictionary *aPersonalDictionary); \
  nsresult GetDictionaryList(nsTArray<nsCString >& _retval); \
  nsresult Check(const nsAString& word, bool *_retval); \
  nsresult Suggest(const nsAString& word, nsTArray<nsString >& _retval); \
  nsresult LoadDictionariesFromDir(nsIFile *dir); \
  nsresult AddDirectory(nsIFile *dir); \
  nsresult RemoveDirectory(nsIFile *dir); \
  nsresult AddDictionary(const nsAString& lang, nsIURI *file); \
  nsresult RemoveDictionary(const nsAString& lang, nsIURI *file, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISPELLCHECKINGENGINE(_to) \
  NS_IMETHOD GetDictionary(nsACString& aDictionary) override { return _to GetDictionary(aDictionary); } \
  NS_IMETHOD SetDictionary(const nsACString& aDictionary) override { return _to SetDictionary(aDictionary); } \
  NS_IMETHOD GetPersonalDictionary(mozIPersonalDictionary **aPersonalDictionary) override { return _to GetPersonalDictionary(aPersonalDictionary); } \
  NS_IMETHOD SetPersonalDictionary(mozIPersonalDictionary *aPersonalDictionary) override { return _to SetPersonalDictionary(aPersonalDictionary); } \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override { return _to GetDictionaryList(_retval); } \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override { return _to Check(word, _retval); } \
  NS_IMETHOD Suggest(const nsAString& word, nsTArray<nsString >& _retval) override { return _to Suggest(word, _retval); } \
  NS_IMETHOD LoadDictionariesFromDir(nsIFile *dir) override { return _to LoadDictionariesFromDir(dir); } \
  NS_IMETHOD AddDirectory(nsIFile *dir) override { return _to AddDirectory(dir); } \
  NS_IMETHOD RemoveDirectory(nsIFile *dir) override { return _to RemoveDirectory(dir); } \
  NS_IMETHOD AddDictionary(const nsAString& lang, nsIURI *file) override { return _to AddDictionary(lang, file); } \
  NS_IMETHOD RemoveDictionary(const nsAString& lang, nsIURI *file, bool *_retval) override { return _to RemoveDictionary(lang, file, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISPELLCHECKINGENGINE(_to) \
  NS_IMETHOD GetDictionary(nsACString& aDictionary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDictionary(aDictionary); } \
  NS_IMETHOD SetDictionary(const nsACString& aDictionary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDictionary(aDictionary); } \
  NS_IMETHOD GetPersonalDictionary(mozIPersonalDictionary **aPersonalDictionary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersonalDictionary(aPersonalDictionary); } \
  NS_IMETHOD SetPersonalDictionary(mozIPersonalDictionary *aPersonalDictionary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersonalDictionary(aPersonalDictionary); } \
  NS_IMETHOD GetDictionaryList(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDictionaryList(_retval); } \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Check(word, _retval); } \
  NS_IMETHOD Suggest(const nsAString& word, nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Suggest(word, _retval); } \
  NS_IMETHOD LoadDictionariesFromDir(nsIFile *dir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadDictionariesFromDir(dir); } \
  NS_IMETHOD AddDirectory(nsIFile *dir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDirectory(dir); } \
  NS_IMETHOD RemoveDirectory(nsIFile *dir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDirectory(dir); } \
  NS_IMETHOD AddDictionary(const nsAString& lang, nsIURI *file) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDictionary(lang, file); } \
  NS_IMETHOD RemoveDictionary(const nsAString& lang, nsIURI *file, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveDictionary(lang, file, _retval); } 

#define SPELLCHECK_DICTIONARY_REMOVE_NOTIFICATION \
  "spellcheck-dictionary-remove"

#endif /* __gen_mozISpellCheckingEngine_h__ */
