/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthModule.idl
 */

#ifndef __gen_nsIAuthModule_h__
#define __gen_nsIAuthModule_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAuthModule */
#define NS_IAUTHMODULE_IID_STR "6e35dbc0-49ef-4e2c-b1ea-b72ec64450a2"

#define NS_IAUTHMODULE_IID \
  {0x6e35dbc0, 0x49ef, 0x4e2c, \
    { 0xb1, 0xea, 0xb7, 0x2e, 0xc6, 0x44, 0x50, 0xa2 }}

class nsIAuthModule : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTHMODULE_IID)

  enum {
    REQ_DEFAULT = 0U,
    REQ_MUTUAL_AUTH = 1U,
    REQ_DELEGATE = 2U,
    REQ_PROXY_AUTH = 4U,
    NTLM_MODULE_SAMBA_AUTH_PROXY = 0U,
    NTLM_MODULE_SAMBA_AUTH_DIRECT = 1U,
    NTLM_MODULE_WIN_API_PROXY = 2U,
    NTLM_MODULE_WIN_API_DIRECT = 3U,
    NTLM_MODULE_GENERIC_PROXY = 4U,
    NTLM_MODULE_GENERIC_DIRECT = 5U,
    NTLM_MODULE_KERBEROS_PROXY = 6U,
    NTLM_MODULE_KERBEROS_DIRECT = 7U
  };

  /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
  NS_IMETHOD Init(const char * aServiceName, uint32_t aServiceFlags, const char16_t * aDomain, const char16_t * aUsername, const char16_t * aPassword) = 0;

  /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
  NS_IMETHOD GetNextToken(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) = 0;

  /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
  NS_IMETHOD Wrap(const void * aInToken, uint32_t aInTokenLength, bool confidential, void * * aOutToken, uint32_t *aOutTokenLength) = 0;

  /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
  NS_IMETHOD Unwrap(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) = 0;

     /**
     * Create a new instance of an auth module.
     *
     * @param aType
     *        The type of the auth module to be constructed.
     */
    static already_AddRefed<nsIAuthModule> CreateInstance(const char* aType);
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAuthModule, NS_IAUTHMODULE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTHMODULE \
  NS_IMETHOD Init(const char * aServiceName, uint32_t aServiceFlags, const char16_t * aDomain, const char16_t * aUsername, const char16_t * aPassword) override; \
  NS_IMETHOD GetNextToken(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override; \
  NS_IMETHOD Wrap(const void * aInToken, uint32_t aInTokenLength, bool confidential, void * * aOutToken, uint32_t *aOutTokenLength) override; \
  NS_IMETHOD Unwrap(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTHMODULE \
  nsresult Init(const char * aServiceName, uint32_t aServiceFlags, const char16_t * aDomain, const char16_t * aUsername, const char16_t * aPassword); \
  nsresult GetNextToken(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength); \
  nsresult Wrap(const void * aInToken, uint32_t aInTokenLength, bool confidential, void * * aOutToken, uint32_t *aOutTokenLength); \
  nsresult Unwrap(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTHMODULE(_to) \
  NS_IMETHOD Init(const char * aServiceName, uint32_t aServiceFlags, const char16_t * aDomain, const char16_t * aUsername, const char16_t * aPassword) override { return _to Init(aServiceName, aServiceFlags, aDomain, aUsername, aPassword); } \
  NS_IMETHOD GetNextToken(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override { return _to GetNextToken(aInToken, aInTokenLength, aOutToken, aOutTokenLength); } \
  NS_IMETHOD Wrap(const void * aInToken, uint32_t aInTokenLength, bool confidential, void * * aOutToken, uint32_t *aOutTokenLength) override { return _to Wrap(aInToken, aInTokenLength, confidential, aOutToken, aOutTokenLength); } \
  NS_IMETHOD Unwrap(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override { return _to Unwrap(aInToken, aInTokenLength, aOutToken, aOutTokenLength); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTHMODULE(_to) \
  NS_IMETHOD Init(const char * aServiceName, uint32_t aServiceFlags, const char16_t * aDomain, const char16_t * aUsername, const char16_t * aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aServiceName, aServiceFlags, aDomain, aUsername, aPassword); } \
  NS_IMETHOD GetNextToken(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextToken(aInToken, aInTokenLength, aOutToken, aOutTokenLength); } \
  NS_IMETHOD Wrap(const void * aInToken, uint32_t aInTokenLength, bool confidential, void * * aOutToken, uint32_t *aOutTokenLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Wrap(aInToken, aInTokenLength, confidential, aOutToken, aOutTokenLength); } \
  NS_IMETHOD Unwrap(const void * aInToken, uint32_t aInTokenLength, void * * aOutToken, uint32_t *aOutTokenLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unwrap(aInToken, aInTokenLength, aOutToken, aOutTokenLength); } \


#endif /* __gen_nsIAuthModule_h__ */
