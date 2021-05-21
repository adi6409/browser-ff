/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthRememberService.idl
 */

#ifndef __gen_nsIClientAuthRememberService_h__
#define __gen_nsIClientAuthRememberService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "cert.h"
#define NS_CLIENTAUTHREMEMBERSERVICE_CONTRACTID "@mozilla.org/security/clientAuthRememberService;1"

/* starting interface:    nsIClientAuthRememberRecord */
#define NS_ICLIENTAUTHREMEMBERRECORD_IID_STR "e92825af-7e81-4b5c-b412-8e1dd36d14fe"

#define NS_ICLIENTAUTHREMEMBERRECORD_IID \
  {0xe92825af, 0x7e81, 0x4b5c, \
    { 0xb4, 0x12, 0x8e, 0x1d, 0xd3, 0x6d, 0x14, 0xfe }}

class NS_NO_VTABLE nsIClientAuthRememberRecord : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIENTAUTHREMEMBERRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClientAuthRememberRecord;

  /* readonly attribute ACString asciiHost; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) = 0;

  /* readonly attribute ACString fingerprint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFingerprint(nsACString& aFingerprint) = 0;

  /* readonly attribute ACString dbKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDbKey(nsACString& aDbKey) = 0;

  /* readonly attribute ACString entryKey; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntryKey(nsACString& aEntryKey) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClientAuthRememberRecord, NS_ICLIENTAUTHREMEMBERRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIENTAUTHREMEMBERRECORD \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override; \
  NS_IMETHOD GetFingerprint(nsACString& aFingerprint) override; \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override; \
  NS_IMETHOD GetEntryKey(nsACString& aEntryKey) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIENTAUTHREMEMBERRECORD \
  nsresult GetAsciiHost(nsACString& aAsciiHost); \
  nsresult GetFingerprint(nsACString& aFingerprint); \
  nsresult GetDbKey(nsACString& aDbKey); \
  nsresult GetEntryKey(nsACString& aEntryKey); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIENTAUTHREMEMBERRECORD(_to) \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return _to GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetFingerprint(nsACString& aFingerprint) override { return _to GetFingerprint(aFingerprint); } \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return _to GetDbKey(aDbKey); } \
  NS_IMETHOD GetEntryKey(nsACString& aEntryKey) override { return _to GetEntryKey(aEntryKey); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIENTAUTHREMEMBERRECORD(_to) \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetFingerprint(nsACString& aFingerprint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFingerprint(aFingerprint); } \
  NS_IMETHOD GetDbKey(nsACString& aDbKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDbKey(aDbKey); } \
  NS_IMETHOD GetEntryKey(nsACString& aEntryKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntryKey(aEntryKey); } 


/* starting interface:    nsIClientAuthRememberService */
#define NS_ICLIENTAUTHREMEMBERSERVICE_IID_STR "1dbc6eb6-0972-4bdb-9dc4-acd0abf72369"

#define NS_ICLIENTAUTHREMEMBERSERVICE_IID \
  {0x1dbc6eb6, 0x0972, 0x4bdb, \
    { 0x9d, 0xc4, 0xac, 0xd0, 0xab, 0xf7, 0x23, 0x69 }}

class NS_NO_VTABLE nsIClientAuthRememberService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLIENTAUTHREMEMBERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClientAuthRememberService;

  /* [must_use] void forgetRememberedDecision (in ACString key); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ForgetRememberedDecision(const nsACString& key) = 0;

  /* [must_use] Array<nsIClientAuthRememberRecord> getDecisions (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetDecisions(nsTArray<RefPtr<nsIClientAuthRememberRecord>>& _retval) = 0;

  /* [must_use,noscript] void rememberDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, in CERTCertificatePtr aClientCert); */
  [[nodiscard]] NS_IMETHOD RememberDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, CERTCertificate * aClientCert) = 0;

  /* [must_use,noscript] bool hasRememberedDecision (in ACString aHostName, in const_OriginAttributesRef aOriginAttributes, in CERTCertificatePtr aServerCert, out ACString aCertDBKey); */
  [[nodiscard]] NS_IMETHOD HasRememberedDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, nsACString& aCertDBKey, bool *_retval) = 0;

  /* [must_use] void clearRememberedDecisions (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ClearRememberedDecisions(void) = 0;

  /* [implicit_jscontext] void deleteDecisionsByHost (in ACString aHostName, in jsval aOriginAttributes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteDecisionsByHost(const nsACString& aHostName, JS::HandleValue aOriginAttributes, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClientAuthRememberService, NS_ICLIENTAUTHREMEMBERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLIENTAUTHREMEMBERSERVICE \
  [[nodiscard]] NS_IMETHOD ForgetRememberedDecision(const nsACString& key) override; \
  [[nodiscard]] NS_IMETHOD GetDecisions(nsTArray<RefPtr<nsIClientAuthRememberRecord>>& _retval) override; \
  [[nodiscard]] NS_IMETHOD RememberDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, CERTCertificate * aClientCert) override; \
  [[nodiscard]] NS_IMETHOD HasRememberedDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, nsACString& aCertDBKey, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD ClearRememberedDecisions(void) override; \
  NS_IMETHOD DeleteDecisionsByHost(const nsACString& aHostName, JS::HandleValue aOriginAttributes, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLIENTAUTHREMEMBERSERVICE \
  [[nodiscard]] nsresult ForgetRememberedDecision(const nsACString& key); \
  [[nodiscard]] nsresult GetDecisions(nsTArray<RefPtr<nsIClientAuthRememberRecord>>& _retval); \
  [[nodiscard]] nsresult RememberDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, CERTCertificate * aClientCert); \
  [[nodiscard]] nsresult HasRememberedDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, nsACString& aCertDBKey, bool *_retval); \
  [[nodiscard]] nsresult ClearRememberedDecisions(void); \
  nsresult DeleteDecisionsByHost(const nsACString& aHostName, JS::HandleValue aOriginAttributes, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLIENTAUTHREMEMBERSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD ForgetRememberedDecision(const nsACString& key) override { return _to ForgetRememberedDecision(key); } \
  [[nodiscard]] NS_IMETHOD GetDecisions(nsTArray<RefPtr<nsIClientAuthRememberRecord>>& _retval) override { return _to GetDecisions(_retval); } \
  [[nodiscard]] NS_IMETHOD RememberDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, CERTCertificate * aClientCert) override { return _to RememberDecision(aHostName, aOriginAttributes, aServerCert, aClientCert); } \
  [[nodiscard]] NS_IMETHOD HasRememberedDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, nsACString& aCertDBKey, bool *_retval) override { return _to HasRememberedDecision(aHostName, aOriginAttributes, aServerCert, aCertDBKey, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearRememberedDecisions(void) override { return _to ClearRememberedDecisions(); } \
  NS_IMETHOD DeleteDecisionsByHost(const nsACString& aHostName, JS::HandleValue aOriginAttributes, JSContext* cx) override { return _to DeleteDecisionsByHost(aHostName, aOriginAttributes, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLIENTAUTHREMEMBERSERVICE(_to) \
  [[nodiscard]] NS_IMETHOD ForgetRememberedDecision(const nsACString& key) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForgetRememberedDecision(key); } \
  [[nodiscard]] NS_IMETHOD GetDecisions(nsTArray<RefPtr<nsIClientAuthRememberRecord>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDecisions(_retval); } \
  [[nodiscard]] NS_IMETHOD RememberDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, CERTCertificate * aClientCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RememberDecision(aHostName, aOriginAttributes, aServerCert, aClientCert); } \
  [[nodiscard]] NS_IMETHOD HasRememberedDecision(const nsACString& aHostName, const mozilla::OriginAttributes & aOriginAttributes, CERTCertificate * aServerCert, nsACString& aCertDBKey, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasRememberedDecision(aHostName, aOriginAttributes, aServerCert, aCertDBKey, _retval); } \
  [[nodiscard]] NS_IMETHOD ClearRememberedDecisions(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearRememberedDecisions(); } \
  NS_IMETHOD DeleteDecisionsByHost(const nsACString& aHostName, JS::HandleValue aOriginAttributes, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteDecisionsByHost(aHostName, aOriginAttributes, cx); } 


#endif /* __gen_nsIClientAuthRememberService_h__ */
