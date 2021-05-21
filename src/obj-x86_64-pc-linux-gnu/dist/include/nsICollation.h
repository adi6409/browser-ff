/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/nsICollation.idl
 */

#ifndef __gen_nsICollation_h__
#define __gen_nsICollation_h__


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
class nsICollation; /* forward declaration */


/* starting interface:    nsICollationFactory */
#define NS_ICOLLATIONFACTORY_IID_STR "04971e14-d6b3-4ada-8cbb-c3a13842b349"

#define NS_ICOLLATIONFACTORY_IID \
  {0x04971e14, 0xd6b3, 0x4ada, \
    { 0x8c, 0xbb, 0xc3, 0xa1, 0x38, 0x42, 0xb3, 0x49 }}

class NS_NO_VTABLE nsICollationFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOLLATIONFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICollationFactory;

  /* nsICollation CreateCollation (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateCollation(nsICollation **_retval) = 0;

  /* nsICollation CreateCollationForLocale (in ACString locale); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateCollationForLocale(const nsACString& locale, nsICollation **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICollationFactory, NS_ICOLLATIONFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOLLATIONFACTORY \
  NS_IMETHOD CreateCollation(nsICollation **_retval) override; \
  NS_IMETHOD CreateCollationForLocale(const nsACString& locale, nsICollation **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOLLATIONFACTORY \
  nsresult CreateCollation(nsICollation **_retval); \
  nsresult CreateCollationForLocale(const nsACString& locale, nsICollation **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOLLATIONFACTORY(_to) \
  NS_IMETHOD CreateCollation(nsICollation **_retval) override { return _to CreateCollation(_retval); } \
  NS_IMETHOD CreateCollationForLocale(const nsACString& locale, nsICollation **_retval) override { return _to CreateCollationForLocale(locale, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOLLATIONFACTORY(_to) \
  NS_IMETHOD CreateCollation(nsICollation **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCollation(_retval); } \
  NS_IMETHOD CreateCollationForLocale(const nsACString& locale, nsICollation **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateCollationForLocale(locale, _retval); } 


/* starting interface:    nsICollation */
#define NS_ICOLLATION_IID_STR "b0132cc0-3786-4557-9874-910d7def5f93"

#define NS_ICOLLATION_IID \
  {0xb0132cc0, 0x3786, 0x4557, \
    { 0x98, 0x74, 0x91, 0x0d, 0x7d, 0xef, 0x5f, 0x93 }}

class NS_NO_VTABLE nsICollation : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOLLATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICollation;

  enum {
    kCollationStrengthDefault = 0,
    kCollationCaseInsensitiveAscii = 1,
    kCollationAccentInsenstive = 2,
    kCollationCaseSensitive = 0,
    kCollationCaseInSensitive = 3
  };

  /* void initialize (in ACString locale); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Initialize(const nsACString& locale) = 0;

  /* long compareString (in long strength, in AString string1, in AString string2); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompareString(int32_t strength, const nsAString& string1, const nsAString& string2, int32_t *_retval) = 0;

  /* [noscript] Array<octet> allocateRawSortKey (in long strength, in AString stringIn); */
  NS_IMETHOD AllocateRawSortKey(int32_t strength, const nsAString& stringIn, nsTArray<uint8_t >& _retval) = 0;

  /* [noscript] long compareRawSortKey (in Array<octet> key1, in Array<octet> key2); */
  NS_IMETHOD CompareRawSortKey(const nsTArray<uint8_t >& key1, const nsTArray<uint8_t >& key2, int32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICollation, NS_ICOLLATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOLLATION \
  NS_IMETHOD Initialize(const nsACString& locale) override; \
  NS_IMETHOD CompareString(int32_t strength, const nsAString& string1, const nsAString& string2, int32_t *_retval) override; \
  NS_IMETHOD AllocateRawSortKey(int32_t strength, const nsAString& stringIn, nsTArray<uint8_t >& _retval) override; \
  NS_IMETHOD CompareRawSortKey(const nsTArray<uint8_t >& key1, const nsTArray<uint8_t >& key2, int32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOLLATION \
  nsresult Initialize(const nsACString& locale); \
  nsresult CompareString(int32_t strength, const nsAString& string1, const nsAString& string2, int32_t *_retval); \
  nsresult AllocateRawSortKey(int32_t strength, const nsAString& stringIn, nsTArray<uint8_t >& _retval); \
  nsresult CompareRawSortKey(const nsTArray<uint8_t >& key1, const nsTArray<uint8_t >& key2, int32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOLLATION(_to) \
  NS_IMETHOD Initialize(const nsACString& locale) override { return _to Initialize(locale); } \
  NS_IMETHOD CompareString(int32_t strength, const nsAString& string1, const nsAString& string2, int32_t *_retval) override { return _to CompareString(strength, string1, string2, _retval); } \
  NS_IMETHOD AllocateRawSortKey(int32_t strength, const nsAString& stringIn, nsTArray<uint8_t >& _retval) override { return _to AllocateRawSortKey(strength, stringIn, _retval); } \
  NS_IMETHOD CompareRawSortKey(const nsTArray<uint8_t >& key1, const nsTArray<uint8_t >& key2, int32_t *_retval) override { return _to CompareRawSortKey(key1, key2, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOLLATION(_to) \
  NS_IMETHOD Initialize(const nsACString& locale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Initialize(locale); } \
  NS_IMETHOD CompareString(int32_t strength, const nsAString& string1, const nsAString& string2, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompareString(strength, string1, string2, _retval); } \
  NS_IMETHOD AllocateRawSortKey(int32_t strength, const nsAString& stringIn, nsTArray<uint8_t >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllocateRawSortKey(strength, stringIn, _retval); } \
  NS_IMETHOD CompareRawSortKey(const nsTArray<uint8_t >& key1, const nsTArray<uint8_t >& key2, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompareRawSortKey(key1, key2, _retval); } 


#endif /* __gen_nsICollation_h__ */
