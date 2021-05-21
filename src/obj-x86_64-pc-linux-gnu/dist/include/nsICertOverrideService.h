/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertOverrideService.idl
 */

#ifndef __gen_nsICertOverrideService_h__
#define __gen_nsICertOverrideService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */

class nsIX509Cert; /* forward declaration */

#define NS_CERTOVERRIDE_CONTRACTID "@mozilla.org/security/certoverride;1"

/* starting interface:    nsICertOverride */
#define NS_ICERTOVERRIDE_IID_STR "ed735e24-fa55-4163-906d-17fb78851fe1"

#define NS_ICERTOVERRIDE_IID \
  {0xed735e24, 0xfa55, 0x4163, \
    { 0x90, 0x6d, 0x17, 0xfb, 0x78, 0x85, 0x1f, 0xe1 }}

class NS_NO_VTABLE nsICertOverride : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTOVERRIDE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertOverride;

  /* readonly attribute ACString asciiHost; */
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) = 0;

  /* readonly attribute int32_t port; */
  NS_IMETHOD GetPort(int32_t *aPort) = 0;

  /* readonly attribute boolean isTemporary; */
  NS_IMETHOD GetIsTemporary(bool *aIsTemporary) = 0;

  /* readonly attribute ACString dbKey; */
  NS_IMETHOD GetDbKey(nsACString& aDbKey) = 0;

  /* readonly attribute ACString hostPort; */
  NS_IMETHOD GetHostPort(nsACString& aHostPort) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertOverride, NS_ICERTOVERRIDE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTOVERRIDE \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override; \
  NS_IMETHOD GetPort(int32_t *aPort) override; \
  NS_IMETHOD GetIsTemporary(bool *aIsTemporary) override; \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override; \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTOVERRIDE \
  nsresult GetAsciiHost(nsACString& aAsciiHost); \
  nsresult GetPort(int32_t *aPort); \
  nsresult GetIsTemporary(bool *aIsTemporary); \
  nsresult GetDbKey(nsACString& aDbKey); \
  nsresult GetHostPort(nsACString& aHostPort); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTOVERRIDE(_to) \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return _to GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetIsTemporary(bool *aIsTemporary) override { return _to GetIsTemporary(aIsTemporary); } \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return _to GetDbKey(aDbKey); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return _to GetHostPort(aHostPort); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTOVERRIDE(_to) \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetPort(int32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetIsTemporary(bool *aIsTemporary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTemporary(aIsTemporary); } \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDbKey(aDbKey); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostPort(aHostPort); } 


/* starting interface:    nsICertOverrideService */
#define NS_ICERTOVERRIDESERVICE_IID_STR "be019e47-22fc-4355-9f16-9ab047d6742d"

#define NS_ICERTOVERRIDESERVICE_IID \
  {0xbe019e47, 0x22fc, 0x4355, \
    { 0x9f, 0x16, 0x9a, 0xb0, 0x47, 0xd6, 0x74, 0x2d }}

class NS_NO_VTABLE nsICertOverrideService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTOVERRIDESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertOverrideService;

  enum {
    ERROR_UNTRUSTED = 1,
    ERROR_MISMATCH = 2,
    ERROR_TIME = 4
  };

  /* [must_use] void rememberValidityOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, in uint32_t aOverrideBits, in boolean aTemporary); */
  [[nodiscard]] NS_IMETHOD RememberValidityOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t aOverrideBits, bool aTemporary) = 0;

  /* [must_use] void rememberTemporaryValidityOverrideUsingFingerprint (in AUTF8String aHostName, in int32_t aPort, in AUTF8String aCertFingerprint, in uint32_t aOverrideBits); */
  [[nodiscard]] NS_IMETHOD RememberTemporaryValidityOverrideUsingFingerprint(const nsACString& aHostName, int32_t aPort, const nsACString& aCertFingerprint, uint32_t aOverrideBits) = 0;

  /* [must_use] boolean hasMatchingOverride (in AUTF8String aHostName, in int32_t aPort, in nsIX509Cert aCert, out uint32_t aOverrideBits, out boolean aIsTemporary); */
  [[nodiscard]] NS_IMETHOD HasMatchingOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t *aOverrideBits, bool *aIsTemporary, bool *_retval) = 0;

  /* void clearValidityOverride (in AUTF8String aHostName, in int32_t aPort); */
  NS_IMETHOD ClearValidityOverride(const nsACString& aHostName, int32_t aPort) = 0;

  /* void clearAllOverrides (); */
  NS_IMETHOD ClearAllOverrides(void) = 0;

  /* [must_use] uint32_t isCertUsedForOverrides (in nsIX509Cert aCert, in boolean aCheckTemporaries, in boolean aCheckPermanents); */
  [[nodiscard]] NS_IMETHOD IsCertUsedForOverrides(nsIX509Cert *aCert, bool aCheckTemporaries, bool aCheckPermanents, uint32_t *_retval) = 0;

  /* Array<nsICertOverride> getOverrides (); */
  NS_IMETHOD GetOverrides(nsTArray<RefPtr<nsICertOverride>>& _retval) = 0;

  /* void setDisableAllSecurityChecksAndLetAttackersInterceptMyData (in boolean aDisable); */
  NS_IMETHOD SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(bool aDisable) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertOverrideService, NS_ICERTOVERRIDESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTOVERRIDESERVICE \
  [[nodiscard]] NS_IMETHOD RememberValidityOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t aOverrideBits, bool aTemporary) override; \
  [[nodiscard]] NS_IMETHOD RememberTemporaryValidityOverrideUsingFingerprint(const nsACString& aHostName, int32_t aPort, const nsACString& aCertFingerprint, uint32_t aOverrideBits) override; \
  [[nodiscard]] NS_IMETHOD HasMatchingOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t *aOverrideBits, bool *aIsTemporary, bool *_retval) override; \
  NS_IMETHOD ClearValidityOverride(const nsACString& aHostName, int32_t aPort) override; \
  NS_IMETHOD ClearAllOverrides(void) override; \
  [[nodiscard]] NS_IMETHOD IsCertUsedForOverrides(nsIX509Cert *aCert, bool aCheckTemporaries, bool aCheckPermanents, uint32_t *_retval) override; \
  NS_IMETHOD GetOverrides(nsTArray<RefPtr<nsICertOverride>>& _retval) override; \
  NS_IMETHOD SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(bool aDisable) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTOVERRIDESERVICE \
  [[nodiscard]] nsresult RememberValidityOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t aOverrideBits, bool aTemporary); \
  [[nodiscard]] nsresult RememberTemporaryValidityOverrideUsingFingerprint(const nsACString& aHostName, int32_t aPort, const nsACString& aCertFingerprint, uint32_t aOverrideBits); \
  [[nodiscard]] nsresult HasMatchingOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t *aOverrideBits, bool *aIsTemporary, bool *_retval); \
  nsresult ClearValidityOverride(const nsACString& aHostName, int32_t aPort); \
  nsresult ClearAllOverrides(void); \
  [[nodiscard]] nsresult IsCertUsedForOverrides(nsIX509Cert *aCert, bool aCheckTemporaries, bool aCheckPermanents, uint32_t *_retval); \
  nsresult GetOverrides(nsTArray<RefPtr<nsICertOverride>>& _retval); \
  nsresult SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(bool aDisable); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTOVERRIDESERVICE(_to) \
  [[nodiscard]] NS_IMETHOD RememberValidityOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t aOverrideBits, bool aTemporary) override { return _to RememberValidityOverride(aHostName, aPort, aCert, aOverrideBits, aTemporary); } \
  [[nodiscard]] NS_IMETHOD RememberTemporaryValidityOverrideUsingFingerprint(const nsACString& aHostName, int32_t aPort, const nsACString& aCertFingerprint, uint32_t aOverrideBits) override { return _to RememberTemporaryValidityOverrideUsingFingerprint(aHostName, aPort, aCertFingerprint, aOverrideBits); } \
  [[nodiscard]] NS_IMETHOD HasMatchingOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t *aOverrideBits, bool *aIsTemporary, bool *_retval) override { return _to HasMatchingOverride(aHostName, aPort, aCert, aOverrideBits, aIsTemporary, _retval); } \
  NS_IMETHOD ClearValidityOverride(const nsACString& aHostName, int32_t aPort) override { return _to ClearValidityOverride(aHostName, aPort); } \
  NS_IMETHOD ClearAllOverrides(void) override { return _to ClearAllOverrides(); } \
  [[nodiscard]] NS_IMETHOD IsCertUsedForOverrides(nsIX509Cert *aCert, bool aCheckTemporaries, bool aCheckPermanents, uint32_t *_retval) override { return _to IsCertUsedForOverrides(aCert, aCheckTemporaries, aCheckPermanents, _retval); } \
  NS_IMETHOD GetOverrides(nsTArray<RefPtr<nsICertOverride>>& _retval) override { return _to GetOverrides(_retval); } \
  NS_IMETHOD SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(bool aDisable) override { return _to SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(aDisable); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTOVERRIDESERVICE(_to) \
  [[nodiscard]] NS_IMETHOD RememberValidityOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t aOverrideBits, bool aTemporary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RememberValidityOverride(aHostName, aPort, aCert, aOverrideBits, aTemporary); } \
  [[nodiscard]] NS_IMETHOD RememberTemporaryValidityOverrideUsingFingerprint(const nsACString& aHostName, int32_t aPort, const nsACString& aCertFingerprint, uint32_t aOverrideBits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RememberTemporaryValidityOverrideUsingFingerprint(aHostName, aPort, aCertFingerprint, aOverrideBits); } \
  [[nodiscard]] NS_IMETHOD HasMatchingOverride(const nsACString& aHostName, int32_t aPort, nsIX509Cert *aCert, uint32_t *aOverrideBits, bool *aIsTemporary, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMatchingOverride(aHostName, aPort, aCert, aOverrideBits, aIsTemporary, _retval); } \
  NS_IMETHOD ClearValidityOverride(const nsACString& aHostName, int32_t aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearValidityOverride(aHostName, aPort); } \
  NS_IMETHOD ClearAllOverrides(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearAllOverrides(); } \
  [[nodiscard]] NS_IMETHOD IsCertUsedForOverrides(nsIX509Cert *aCert, bool aCheckTemporaries, bool aCheckPermanents, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsCertUsedForOverrides(aCert, aCheckTemporaries, aCheckPermanents, _retval); } \
  NS_IMETHOD GetOverrides(nsTArray<RefPtr<nsICertOverride>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOverrides(_retval); } \
  NS_IMETHOD SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(bool aDisable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisableAllSecurityChecksAndLetAttackersInterceptMyData(aDisable); } 


#endif /* __gen_nsICertOverrideService_h__ */
