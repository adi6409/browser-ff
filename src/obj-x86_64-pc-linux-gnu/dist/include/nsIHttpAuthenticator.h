/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthenticator.idl
 */

#ifndef __gen_nsIHttpAuthenticator_h__
#define __gen_nsIHttpAuthenticator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIHttpAuthenticableChannel; /* forward declaration */

class nsIHttpAuthenticatorCallback; /* forward declaration */

class nsICancelable; /* forward declaration */


/* starting interface:    nsIHttpAuthenticator */
#define NS_IHTTPAUTHENTICATOR_IID_STR "fef7db8a-a4e2-49d1-9685-19ed7e309b7d"

#define NS_IHTTPAUTHENTICATOR_IID \
  {0xfef7db8a, 0xa4e2, 0x49d1, \
    { 0x96, 0x85, 0x19, 0xed, 0x7e, 0x30, 0x9b, 0x7d }}

class NS_NO_VTABLE nsIHttpAuthenticator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPAUTHENTICATOR_IID)

  /* [must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity); */
  [[nodiscard]] NS_IMETHOD ChallengeReceived(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, nsISupports **aSessionState, nsISupports **aContinuationState, bool *aInvalidatesIdentity) = 0;

  /* [must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel); */
  [[nodiscard]] NS_IMETHOD GenerateCredentialsAsync(nsIHttpAuthenticableChannel *aChannel, nsIHttpAuthenticatorCallback *aCallback, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports *aSessionState, nsISupports *aContinuationState, nsICancelable **aCancel) = 0;

  /* [must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags); */
  [[nodiscard]] NS_IMETHOD GenerateCredentials(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports **aSessionState, nsISupports **aContinuationState, uint32_t *aFlags, char * *_retval) = 0;

  enum {
    USING_INTERNAL_IDENTITY = 1U
  };

  /* [must_use] readonly attribute unsigned long authFlags; */
  [[nodiscard]] NS_IMETHOD GetAuthFlags(uint32_t *aAuthFlags) = 0;

  enum {
    REQUEST_BASED = 1U,
    CONNECTION_BASED = 2U,
    REUSABLE_CREDENTIALS = 4U,
    REUSABLE_CHALLENGE = 8U,
    IDENTITY_IGNORED = 1024U,
    IDENTITY_INCLUDES_DOMAIN = 2048U,
    IDENTITY_ENCRYPTED = 4096U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpAuthenticator, NS_IHTTPAUTHENTICATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPAUTHENTICATOR \
  [[nodiscard]] NS_IMETHOD ChallengeReceived(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, nsISupports **aSessionState, nsISupports **aContinuationState, bool *aInvalidatesIdentity) override; \
  [[nodiscard]] NS_IMETHOD GenerateCredentialsAsync(nsIHttpAuthenticableChannel *aChannel, nsIHttpAuthenticatorCallback *aCallback, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports *aSessionState, nsISupports *aContinuationState, nsICancelable **aCancel) override; \
  [[nodiscard]] NS_IMETHOD GenerateCredentials(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports **aSessionState, nsISupports **aContinuationState, uint32_t *aFlags, char * *_retval) override; \
  [[nodiscard]] NS_IMETHOD GetAuthFlags(uint32_t *aAuthFlags) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPAUTHENTICATOR \
  [[nodiscard]] nsresult ChallengeReceived(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, nsISupports **aSessionState, nsISupports **aContinuationState, bool *aInvalidatesIdentity); \
  [[nodiscard]] nsresult GenerateCredentialsAsync(nsIHttpAuthenticableChannel *aChannel, nsIHttpAuthenticatorCallback *aCallback, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports *aSessionState, nsISupports *aContinuationState, nsICancelable **aCancel); \
  [[nodiscard]] nsresult GenerateCredentials(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports **aSessionState, nsISupports **aContinuationState, uint32_t *aFlags, char * *_retval); \
  [[nodiscard]] nsresult GetAuthFlags(uint32_t *aAuthFlags); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPAUTHENTICATOR(_to) \
  [[nodiscard]] NS_IMETHOD ChallengeReceived(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, nsISupports **aSessionState, nsISupports **aContinuationState, bool *aInvalidatesIdentity) override { return _to ChallengeReceived(aChannel, aChallenge, aProxyAuth, aSessionState, aContinuationState, aInvalidatesIdentity); } \
  [[nodiscard]] NS_IMETHOD GenerateCredentialsAsync(nsIHttpAuthenticableChannel *aChannel, nsIHttpAuthenticatorCallback *aCallback, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports *aSessionState, nsISupports *aContinuationState, nsICancelable **aCancel) override { return _to GenerateCredentialsAsync(aChannel, aCallback, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aCancel); } \
  [[nodiscard]] NS_IMETHOD GenerateCredentials(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports **aSessionState, nsISupports **aContinuationState, uint32_t *aFlags, char * *_retval) override { return _to GenerateCredentials(aChannel, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aFlags, _retval); } \
  [[nodiscard]] NS_IMETHOD GetAuthFlags(uint32_t *aAuthFlags) override { return _to GetAuthFlags(aAuthFlags); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPAUTHENTICATOR(_to) \
  [[nodiscard]] NS_IMETHOD ChallengeReceived(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, nsISupports **aSessionState, nsISupports **aContinuationState, bool *aInvalidatesIdentity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ChallengeReceived(aChannel, aChallenge, aProxyAuth, aSessionState, aContinuationState, aInvalidatesIdentity); } \
  [[nodiscard]] NS_IMETHOD GenerateCredentialsAsync(nsIHttpAuthenticableChannel *aChannel, nsIHttpAuthenticatorCallback *aCallback, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports *aSessionState, nsISupports *aContinuationState, nsICancelable **aCancel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateCredentialsAsync(aChannel, aCallback, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aCancel); } \
  [[nodiscard]] NS_IMETHOD GenerateCredentials(nsIHttpAuthenticableChannel *aChannel, const char * aChallenge, bool aProxyAuth, const char16_t * aDomain, const char16_t * aUser, const char16_t * aPassword, nsISupports **aSessionState, nsISupports **aContinuationState, uint32_t *aFlags, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateCredentials(aChannel, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aFlags, _retval); } \
  [[nodiscard]] NS_IMETHOD GetAuthFlags(uint32_t *aAuthFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAuthFlags(aAuthFlags); } \


#endif /* __gen_nsIHttpAuthenticator_h__ */
