/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStandardURL.idl
 */

#ifndef __gen_nsIStandardURL_h__
#define __gen_nsIStandardURL_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIURIMutator; /* forward declaration */


/* starting interface:    nsIStandardURL */
#define NS_ISTANDARDURL_IID_STR "babd6cca-ebe7-4329-967c-d6b9e33caa81"

#define NS_ISTANDARDURL_IID \
  {0xbabd6cca, 0xebe7, 0x4329, \
    { 0x96, 0x7c, 0xd6, 0xb9, 0xe3, 0x3c, 0xaa, 0x81 }}

class NS_NO_VTABLE nsIStandardURL : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTANDARDURL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStandardURL;

  enum {
    URLTYPE_STANDARD = 1U,
    URLTYPE_AUTHORITY = 2U,
    URLTYPE_NO_AUTHORITY = 3U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStandardURL, NS_ISTANDARDURL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTANDARDURL \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTANDARDURL \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTANDARDURL(_to) \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTANDARDURL(_to) \


/* starting interface:    nsIStandardURLMutator */
#define NS_ISTANDARDURLMUTATOR_IID_STR "fc894e98-23a1-43cd-a7fe-72876f8ea2ee"

#define NS_ISTANDARDURLMUTATOR_IID \
  {0xfc894e98, 0x23a1, 0x43cd, \
    { 0xa7, 0xfe, 0x72, 0x87, 0x6f, 0x8e, 0xa2, 0xee }}

class NS_NO_VTABLE nsIStandardURLMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTANDARDURLMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStandardURLMutator;

  /* nsIURIMutator init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
  NS_IMETHOD Init(uint32_t aUrlType, int32_t aDefaultPort, const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURIMutator **_retval) = 0;

  /* nsIURIMutator setDefaultPort (in long aNewDefaultPort); */
  NS_IMETHOD SetDefaultPort(int32_t aNewDefaultPort, nsIURIMutator **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStandardURLMutator, NS_ISTANDARDURLMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTANDARDURLMUTATOR \
  NS_IMETHOD Init(uint32_t aUrlType, int32_t aDefaultPort, const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURIMutator **_retval) override; \
  NS_IMETHOD SetDefaultPort(int32_t aNewDefaultPort, nsIURIMutator **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTANDARDURLMUTATOR \
  nsresult Init(uint32_t aUrlType, int32_t aDefaultPort, const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURIMutator **_retval); \
  nsresult SetDefaultPort(int32_t aNewDefaultPort, nsIURIMutator **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTANDARDURLMUTATOR(_to) \
  NS_IMETHOD Init(uint32_t aUrlType, int32_t aDefaultPort, const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURIMutator **_retval) override { return _to Init(aUrlType, aDefaultPort, aSpec, aOriginCharset, aBaseURI, _retval); } \
  NS_IMETHOD SetDefaultPort(int32_t aNewDefaultPort, nsIURIMutator **_retval) override { return _to SetDefaultPort(aNewDefaultPort, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTANDARDURLMUTATOR(_to) \
  NS_IMETHOD Init(uint32_t aUrlType, int32_t aDefaultPort, const nsACString& aSpec, const char * aOriginCharset, nsIURI *aBaseURI, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aUrlType, aDefaultPort, aSpec, aOriginCharset, aBaseURI, _retval); } \
  NS_IMETHOD SetDefaultPort(int32_t aNewDefaultPort, nsIURIMutator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultPort(aNewDefaultPort, _retval); } 


#endif /* __gen_nsIStandardURL_h__ */
