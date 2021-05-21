/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierPrefixSet.idl
 */

#ifndef __gen_nsIUrlClassifierPrefixSet_h__
#define __gen_nsIUrlClassifierPrefixSet_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIFile_h__
#include "nsIFile.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUrlClassifierPrefixSet */
#define NS_IURLCLASSIFIERPREFIXSET_IID_STR "3d8579f0-75fa-4e00-ba41-38661d5b5d17"

#define NS_IURLCLASSIFIERPREFIXSET_IID \
  {0x3d8579f0, 0x75fa, 0x4e00, \
    { 0xba, 0x41, 0x38, 0x66, 0x1d, 0x5b, 0x5d, 0x17 }}

class NS_NO_VTABLE nsIUrlClassifierPrefixSet : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERPREFIXSET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierPrefixSet;

  /* void init (in ACString aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(const nsACString& aName) = 0;

  /* void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrefixes(const uint32_t *aPrefixes, uint32_t aLength) = 0;

  /* void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrefixes(uint32_t *aCount, uint32_t **aPrefixes) = 0;

  /* boolean contains (in unsigned long aPrefix); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Contains(uint32_t aPrefix, bool *_retval) = 0;

  /* boolean isEmpty (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsEmpty(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierPrefixSet, NS_IURLCLASSIFIERPREFIXSET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERPREFIXSET \
  NS_IMETHOD Init(const nsACString& aName) override; \
  NS_IMETHOD SetPrefixes(const uint32_t *aPrefixes, uint32_t aLength) override; \
  NS_IMETHOD GetPrefixes(uint32_t *aCount, uint32_t **aPrefixes) override; \
  NS_IMETHOD Contains(uint32_t aPrefix, bool *_retval) override; \
  NS_IMETHOD IsEmpty(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERPREFIXSET \
  nsresult Init(const nsACString& aName); \
  nsresult SetPrefixes(const uint32_t *aPrefixes, uint32_t aLength); \
  nsresult GetPrefixes(uint32_t *aCount, uint32_t **aPrefixes); \
  nsresult Contains(uint32_t aPrefix, bool *_retval); \
  nsresult IsEmpty(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERPREFIXSET(_to) \
  NS_IMETHOD Init(const nsACString& aName) override { return _to Init(aName); } \
  NS_IMETHOD SetPrefixes(const uint32_t *aPrefixes, uint32_t aLength) override { return _to SetPrefixes(aPrefixes, aLength); } \
  NS_IMETHOD GetPrefixes(uint32_t *aCount, uint32_t **aPrefixes) override { return _to GetPrefixes(aCount, aPrefixes); } \
  NS_IMETHOD Contains(uint32_t aPrefix, bool *_retval) override { return _to Contains(aPrefix, _retval); } \
  NS_IMETHOD IsEmpty(bool *_retval) override { return _to IsEmpty(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERPREFIXSET(_to) \
  NS_IMETHOD Init(const nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aName); } \
  NS_IMETHOD SetPrefixes(const uint32_t *aPrefixes, uint32_t aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrefixes(aPrefixes, aLength); } \
  NS_IMETHOD GetPrefixes(uint32_t *aCount, uint32_t **aPrefixes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrefixes(aCount, aPrefixes); } \
  NS_IMETHOD Contains(uint32_t aPrefix, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Contains(aPrefix, _retval); } \
  NS_IMETHOD IsEmpty(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsEmpty(_retval); } 


#endif /* __gen_nsIUrlClassifierPrefixSet_h__ */
