/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSVersion.idl
 */

#ifndef __gen_nsINSSVersion_h__
#define __gen_nsINSSVersion_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_NSSVERSION_CONTRACTID "@mozilla.org/security/nssversion;1"

/* starting interface:    nsINSSVersion */
#define NS_INSSVERSION_IID_STR "a8a53a2b-75cc-4c68-a9bb-9791dbddaa00"

#define NS_INSSVERSION_IID \
  {0xa8a53a2b, 0x75cc, 0x4c68, \
    { 0xa9, 0xbb, 0x97, 0x91, 0xdb, 0xdd, 0xaa, 0x00 }}

class NS_NO_VTABLE nsINSSVersion : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INSSVERSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINSSVersion;

  /* [must_use] readonly attribute AString NSPR_MinVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSPR_MinVersion(nsAString& aNSPR_MinVersion) = 0;

  /* [must_use] readonly attribute AString NSS_MinVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSS_MinVersion(nsAString& aNSS_MinVersion) = 0;

  /* [must_use] readonly attribute AString NSSUTIL_MinVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSUTIL_MinVersion(nsAString& aNSSUTIL_MinVersion) = 0;

  /* [must_use] readonly attribute AString NSSSSL_MinVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSSSL_MinVersion(nsAString& aNSSSSL_MinVersion) = 0;

  /* [must_use] readonly attribute AString NSSSMIME_MinVersion; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSSMIME_MinVersion(nsAString& aNSSSMIME_MinVersion) = 0;

  /* [must_use] readonly attribute AString NSPR_Version; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSPR_Version(nsAString& aNSPR_Version) = 0;

  /* [must_use] readonly attribute AString NSS_Version; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSS_Version(nsAString& aNSS_Version) = 0;

  /* [must_use] readonly attribute AString NSSUTIL_Version; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSUTIL_Version(nsAString& aNSSUTIL_Version) = 0;

  /* [must_use] readonly attribute AString NSSSSL_Version; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSSSL_Version(nsAString& aNSSSSL_Version) = 0;

  /* [must_use] readonly attribute AString NSSSMIME_Version; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetNSSSMIME_Version(nsAString& aNSSSMIME_Version) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINSSVersion, NS_INSSVERSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINSSVERSION \
  [[nodiscard]] NS_IMETHOD GetNSPR_MinVersion(nsAString& aNSPR_MinVersion) override; \
  [[nodiscard]] NS_IMETHOD GetNSS_MinVersion(nsAString& aNSS_MinVersion) override; \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_MinVersion(nsAString& aNSSUTIL_MinVersion) override; \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_MinVersion(nsAString& aNSSSSL_MinVersion) override; \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_MinVersion(nsAString& aNSSSMIME_MinVersion) override; \
  [[nodiscard]] NS_IMETHOD GetNSPR_Version(nsAString& aNSPR_Version) override; \
  [[nodiscard]] NS_IMETHOD GetNSS_Version(nsAString& aNSS_Version) override; \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_Version(nsAString& aNSSUTIL_Version) override; \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_Version(nsAString& aNSSSSL_Version) override; \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_Version(nsAString& aNSSSMIME_Version) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINSSVERSION \
  [[nodiscard]] nsresult GetNSPR_MinVersion(nsAString& aNSPR_MinVersion); \
  [[nodiscard]] nsresult GetNSS_MinVersion(nsAString& aNSS_MinVersion); \
  [[nodiscard]] nsresult GetNSSUTIL_MinVersion(nsAString& aNSSUTIL_MinVersion); \
  [[nodiscard]] nsresult GetNSSSSL_MinVersion(nsAString& aNSSSSL_MinVersion); \
  [[nodiscard]] nsresult GetNSSSMIME_MinVersion(nsAString& aNSSSMIME_MinVersion); \
  [[nodiscard]] nsresult GetNSPR_Version(nsAString& aNSPR_Version); \
  [[nodiscard]] nsresult GetNSS_Version(nsAString& aNSS_Version); \
  [[nodiscard]] nsresult GetNSSUTIL_Version(nsAString& aNSSUTIL_Version); \
  [[nodiscard]] nsresult GetNSSSSL_Version(nsAString& aNSSSSL_Version); \
  [[nodiscard]] nsresult GetNSSSMIME_Version(nsAString& aNSSSMIME_Version); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINSSVERSION(_to) \
  [[nodiscard]] NS_IMETHOD GetNSPR_MinVersion(nsAString& aNSPR_MinVersion) override { return _to GetNSPR_MinVersion(aNSPR_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSS_MinVersion(nsAString& aNSS_MinVersion) override { return _to GetNSS_MinVersion(aNSS_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_MinVersion(nsAString& aNSSUTIL_MinVersion) override { return _to GetNSSUTIL_MinVersion(aNSSUTIL_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_MinVersion(nsAString& aNSSSSL_MinVersion) override { return _to GetNSSSSL_MinVersion(aNSSSSL_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_MinVersion(nsAString& aNSSSMIME_MinVersion) override { return _to GetNSSSMIME_MinVersion(aNSSSMIME_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSPR_Version(nsAString& aNSPR_Version) override { return _to GetNSPR_Version(aNSPR_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSS_Version(nsAString& aNSS_Version) override { return _to GetNSS_Version(aNSS_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_Version(nsAString& aNSSUTIL_Version) override { return _to GetNSSUTIL_Version(aNSSUTIL_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_Version(nsAString& aNSSSSL_Version) override { return _to GetNSSSSL_Version(aNSSSSL_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_Version(nsAString& aNSSSMIME_Version) override { return _to GetNSSSMIME_Version(aNSSSMIME_Version); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINSSVERSION(_to) \
  [[nodiscard]] NS_IMETHOD GetNSPR_MinVersion(nsAString& aNSPR_MinVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSPR_MinVersion(aNSPR_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSS_MinVersion(nsAString& aNSS_MinVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSS_MinVersion(aNSS_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_MinVersion(nsAString& aNSSUTIL_MinVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSUTIL_MinVersion(aNSSUTIL_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_MinVersion(nsAString& aNSSSSL_MinVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSSSL_MinVersion(aNSSSSL_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_MinVersion(nsAString& aNSSSMIME_MinVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSSMIME_MinVersion(aNSSSMIME_MinVersion); } \
  [[nodiscard]] NS_IMETHOD GetNSPR_Version(nsAString& aNSPR_Version) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSPR_Version(aNSPR_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSS_Version(nsAString& aNSS_Version) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSS_Version(aNSS_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSUTIL_Version(nsAString& aNSSUTIL_Version) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSUTIL_Version(aNSSUTIL_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSSSL_Version(nsAString& aNSSSSL_Version) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSSSL_Version(aNSSSSL_Version); } \
  [[nodiscard]] NS_IMETHOD GetNSSSMIME_Version(nsAString& aNSSSMIME_Version) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNSSSMIME_Version(aNSSSMIME_Version); } 


#endif /* __gen_nsINSSVersion_h__ */
