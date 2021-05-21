/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozIPersonalDictionary.idl
 */

#ifndef __gen_mozIPersonalDictionary_h__
#define __gen_mozIPersonalDictionary_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIStringEnumerator; /* forward declaration */


/* starting interface:    mozIPersonalDictionary */
#define MOZIPERSONALDICTIONARY_IID_STR "7ef52eaf-b7e1-462b-87e2-5d1dbaca9048"

#define MOZIPERSONALDICTIONARY_IID \
  {0x7ef52eaf, 0xb7e1, 0x462b, \
    { 0x87, 0xe2, 0x5d, 0x1d, 0xba, 0xca, 0x90, 0x48 }}

class NS_NO_VTABLE mozIPersonalDictionary : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIPERSONALDICTIONARY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIPersonalDictionary;

  /* void load (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Load(void) = 0;

  /* void save (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Save(void) = 0;

  /* readonly attribute nsIStringEnumerator wordList; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWordList(nsIStringEnumerator **aWordList) = 0;

  /* boolean check (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Check(const nsAString& word, bool *_retval) = 0;

  /* void addWord (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddWord(const nsAString& word) = 0;

  /* void removeWord (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveWord(const nsAString& word) = 0;

  /* void ignoreWord (in AString word); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IgnoreWord(const nsAString& word) = 0;

  /* void endSession (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EndSession(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIPersonalDictionary, MOZIPERSONALDICTIONARY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIPERSONALDICTIONARY \
  NS_IMETHOD Load(void) override; \
  NS_IMETHOD Save(void) override; \
  NS_IMETHOD GetWordList(nsIStringEnumerator **aWordList) override; \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override; \
  NS_IMETHOD AddWord(const nsAString& word) override; \
  NS_IMETHOD RemoveWord(const nsAString& word) override; \
  NS_IMETHOD IgnoreWord(const nsAString& word) override; \
  NS_IMETHOD EndSession(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIPERSONALDICTIONARY \
  nsresult Load(void); \
  nsresult Save(void); \
  nsresult GetWordList(nsIStringEnumerator **aWordList); \
  nsresult Check(const nsAString& word, bool *_retval); \
  nsresult AddWord(const nsAString& word); \
  nsresult RemoveWord(const nsAString& word); \
  nsresult IgnoreWord(const nsAString& word); \
  nsresult EndSession(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIPERSONALDICTIONARY(_to) \
  NS_IMETHOD Load(void) override { return _to Load(); } \
  NS_IMETHOD Save(void) override { return _to Save(); } \
  NS_IMETHOD GetWordList(nsIStringEnumerator **aWordList) override { return _to GetWordList(aWordList); } \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override { return _to Check(word, _retval); } \
  NS_IMETHOD AddWord(const nsAString& word) override { return _to AddWord(word); } \
  NS_IMETHOD RemoveWord(const nsAString& word) override { return _to RemoveWord(word); } \
  NS_IMETHOD IgnoreWord(const nsAString& word) override { return _to IgnoreWord(word); } \
  NS_IMETHOD EndSession(void) override { return _to EndSession(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIPERSONALDICTIONARY(_to) \
  NS_IMETHOD Load(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Load(); } \
  NS_IMETHOD Save(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Save(); } \
  NS_IMETHOD GetWordList(nsIStringEnumerator **aWordList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWordList(aWordList); } \
  NS_IMETHOD Check(const nsAString& word, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Check(word, _retval); } \
  NS_IMETHOD AddWord(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWord(word); } \
  NS_IMETHOD RemoveWord(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWord(word); } \
  NS_IMETHOD IgnoreWord(const nsAString& word) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IgnoreWord(word); } \
  NS_IMETHOD EndSession(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EndSession(); } 


#endif /* __gen_mozIPersonalDictionary_h__ */
