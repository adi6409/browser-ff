/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIReferrerInfo.idl
 */

#ifndef __gen_nsIReferrerInfo_h__
#define __gen_nsIReferrerInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsISerializable_h__
#include "nsISerializable.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla::dom {
enum class ReferrerPolicy : uint8_t;
}
class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIReferrerInfo */
#define NS_IREFERRERINFO_IID_STR "081cdc36-f2e2-4f94-87bf-78578f06f1eb"

#define NS_IREFERRERINFO_IID \
  {0x081cdc36, 0xf2e2, 0x4f94, \
    { 0x87, 0xbf, 0x78, 0x57, 0x8f, 0x06, 0xf1, 0xeb }}

class NS_NO_VTABLE nsIReferrerInfo : public nsISerializable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREFERRERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIReferrerInfo;

  enum ReferrerPolicyIDL : uint8_t {
    EMPTY = 0,
    NO_REFERRER_WHEN_DOWNGRADE = 1,
    NO_REFERRER = 2,
    ORIGIN = 3,
    ORIGIN_WHEN_CROSS_ORIGIN = 4,
    UNSAFE_URL = 5,
    SAME_ORIGIN = 6,
    STRICT_ORIGIN = 7,
    STRICT_ORIGIN_WHEN_CROSS_ORIGIN = 8,
  };

  /* [infallible] readonly attribute nsIURI originalReferrer; */
  NS_IMETHOD GetOriginalReferrer(nsIURI **aOriginalReferrer) = 0;
   inline already_AddRefed<nsIURI> GetOriginalReferrer()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetOriginalReferrer(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }

  /* [implicit_jscontext] readonly attribute nsIReferrerInfo_ReferrerPolicyIDL referrerPolicy; */
  NS_IMETHOD GetReferrerPolicy(JSContext* cx, nsIReferrerInfo::ReferrerPolicyIDL *aReferrerPolicy) = 0;

  /* [binaryname(ReferrerPolicy),noscript,nostdcall,notxpcom] ReferrerPolicy binaryReferrerPolicy (); */
  virtual mozilla::dom::ReferrerPolicy ReferrerPolicy(void) = 0;

  /* ACString getReferrerPolicyString (); */
  NS_IMETHOD GetReferrerPolicyString(nsACString& _retval) = 0;

  /* [infallible] readonly attribute boolean sendReferrer; */
  NS_IMETHOD GetSendReferrer(bool *aSendReferrer) = 0;
  inline bool  GetSendReferrer()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSendReferrer(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute AString computedReferrerSpec; */
  NS_IMETHOD GetComputedReferrerSpec(nsAString& aComputedReferrerSpec) = 0;

  /* [must_use,noscript,nostdcall,notxpcom] URIRef GetComputedReferrer (); */
  [[nodiscard]] virtual already_AddRefed<nsIURI> GetComputedReferrer(void) = 0;

  /* boolean equals (in nsIReferrerInfo other); */
  NS_IMETHOD Equals(nsIReferrerInfo *other, bool *_retval) = 0;

  /* void init (in nsIReferrerInfo_ReferrerPolicyIDL aReferrerPolicy, [optional] in boolean aSendReferrer, [optional] in nsIURI aOriginalReferrer); */
  NS_IMETHOD Init(nsIReferrerInfo::ReferrerPolicyIDL aReferrerPolicy, bool aSendReferrer, nsIURI *aOriginalReferrer) = 0;

  /* void initWithDocument ([const] in Document aDocument); */
  NS_IMETHOD InitWithDocument(const mozilla::dom::Document *aDocument) = 0;

  /* void initWithElement ([const] in Element aNode); */
  NS_IMETHOD InitWithElement(const mozilla::dom::Element *aNode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIReferrerInfo, NS_IREFERRERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREFERRERINFO \
  using nsIReferrerInfo::GetOriginalReferrer; \
  NS_IMETHOD GetOriginalReferrer(nsIURI **aOriginalReferrer) override; \
  NS_IMETHOD GetReferrerPolicy(JSContext* cx, nsIReferrerInfo::ReferrerPolicyIDL *aReferrerPolicy) override; \
  virtual mozilla::dom::ReferrerPolicy ReferrerPolicy(void) override; \
  NS_IMETHOD GetReferrerPolicyString(nsACString& _retval) override; \
  using nsIReferrerInfo::GetSendReferrer; \
  NS_IMETHOD GetSendReferrer(bool *aSendReferrer) override; \
  NS_IMETHOD GetComputedReferrerSpec(nsAString& aComputedReferrerSpec) override; \
  [[nodiscard]] virtual already_AddRefed<nsIURI> GetComputedReferrer(void) override; \
  NS_IMETHOD Equals(nsIReferrerInfo *other, bool *_retval) override; \
  NS_IMETHOD Init(nsIReferrerInfo::ReferrerPolicyIDL aReferrerPolicy, bool aSendReferrer, nsIURI *aOriginalReferrer) override; \
  NS_IMETHOD InitWithDocument(const mozilla::dom::Document *aDocument) override; \
  NS_IMETHOD InitWithElement(const mozilla::dom::Element *aNode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREFERRERINFO \
  using nsIReferrerInfo::GetOriginalReferrer; \
  nsresult GetOriginalReferrer(nsIURI **aOriginalReferrer); \
  nsresult GetReferrerPolicy(JSContext* cx, nsIReferrerInfo::ReferrerPolicyIDL *aReferrerPolicy); \
  mozilla::dom::ReferrerPolicy ReferrerPolicy(void); \
  nsresult GetReferrerPolicyString(nsACString& _retval); \
  using nsIReferrerInfo::GetSendReferrer; \
  nsresult GetSendReferrer(bool *aSendReferrer); \
  nsresult GetComputedReferrerSpec(nsAString& aComputedReferrerSpec); \
  [[nodiscard]] already_AddRefed<nsIURI> GetComputedReferrer(void); \
  nsresult Equals(nsIReferrerInfo *other, bool *_retval); \
  nsresult Init(nsIReferrerInfo::ReferrerPolicyIDL aReferrerPolicy, bool aSendReferrer, nsIURI *aOriginalReferrer); \
  nsresult InitWithDocument(const mozilla::dom::Document *aDocument); \
  nsresult InitWithElement(const mozilla::dom::Element *aNode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREFERRERINFO(_to) \
  using nsIReferrerInfo::GetOriginalReferrer; \
  NS_IMETHOD GetOriginalReferrer(nsIURI **aOriginalReferrer) override { return _to GetOriginalReferrer(aOriginalReferrer); } \
  NS_IMETHOD GetReferrerPolicy(JSContext* cx, nsIReferrerInfo::ReferrerPolicyIDL *aReferrerPolicy) override { return _to GetReferrerPolicy(cx, aReferrerPolicy); } \
  virtual mozilla::dom::ReferrerPolicy ReferrerPolicy(void) override { return _to ReferrerPolicy(); } \
  NS_IMETHOD GetReferrerPolicyString(nsACString& _retval) override { return _to GetReferrerPolicyString(_retval); } \
  using nsIReferrerInfo::GetSendReferrer; \
  NS_IMETHOD GetSendReferrer(bool *aSendReferrer) override { return _to GetSendReferrer(aSendReferrer); } \
  NS_IMETHOD GetComputedReferrerSpec(nsAString& aComputedReferrerSpec) override { return _to GetComputedReferrerSpec(aComputedReferrerSpec); } \
  [[nodiscard]] virtual already_AddRefed<nsIURI> GetComputedReferrer(void) override { return _to GetComputedReferrer(); } \
  NS_IMETHOD Equals(nsIReferrerInfo *other, bool *_retval) override { return _to Equals(other, _retval); } \
  NS_IMETHOD Init(nsIReferrerInfo::ReferrerPolicyIDL aReferrerPolicy, bool aSendReferrer, nsIURI *aOriginalReferrer) override { return _to Init(aReferrerPolicy, aSendReferrer, aOriginalReferrer); } \
  NS_IMETHOD InitWithDocument(const mozilla::dom::Document *aDocument) override { return _to InitWithDocument(aDocument); } \
  NS_IMETHOD InitWithElement(const mozilla::dom::Element *aNode) override { return _to InitWithElement(aNode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREFERRERINFO(_to) \
  NS_IMETHOD GetOriginalReferrer(nsIURI **aOriginalReferrer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalReferrer(aOriginalReferrer); } \
  NS_IMETHOD GetReferrerPolicy(JSContext* cx, nsIReferrerInfo::ReferrerPolicyIDL *aReferrerPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerPolicy(cx, aReferrerPolicy); } \
  virtual mozilla::dom::ReferrerPolicy ReferrerPolicy(void) override; \
  NS_IMETHOD GetReferrerPolicyString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerPolicyString(_retval); } \
  NS_IMETHOD GetSendReferrer(bool *aSendReferrer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendReferrer(aSendReferrer); } \
  NS_IMETHOD GetComputedReferrerSpec(nsAString& aComputedReferrerSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetComputedReferrerSpec(aComputedReferrerSpec); } \
  [[nodiscard]] virtual already_AddRefed<nsIURI> GetComputedReferrer(void) override; \
  NS_IMETHOD Equals(nsIReferrerInfo *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(other, _retval); } \
  NS_IMETHOD Init(nsIReferrerInfo::ReferrerPolicyIDL aReferrerPolicy, bool aSendReferrer, nsIURI *aOriginalReferrer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aReferrerPolicy, aSendReferrer, aOriginalReferrer); } \
  NS_IMETHOD InitWithDocument(const mozilla::dom::Document *aDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithDocument(aDocument); } \
  NS_IMETHOD InitWithElement(const mozilla::dom::Element *aNode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithElement(aNode); } 


#endif /* __gen_nsIReferrerInfo_h__ */
