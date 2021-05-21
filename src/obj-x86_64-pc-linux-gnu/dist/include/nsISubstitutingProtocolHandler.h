/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/res/nsISubstitutingProtocolHandler.idl
 */

#ifndef __gen_nsISubstitutingProtocolHandler_h__
#define __gen_nsISubstitutingProtocolHandler_h__


#ifndef __gen_nsIProtocolHandler_h__
#include "nsIProtocolHandler.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISubstitutingProtocolHandler */
#define NS_ISUBSTITUTINGPROTOCOLHANDLER_IID_STR "154c64fd-a69e-4105-89f8-bd7dfe621372"

#define NS_ISUBSTITUTINGPROTOCOLHANDLER_IID \
  {0x154c64fd, 0xa69e, 0x4105, \
    { 0x89, 0xf8, 0xbd, 0x7d, 0xfe, 0x62, 0x13, 0x72 }}

class NS_NO_VTABLE nsISubstitutingProtocolHandler : public nsIProtocolHandler {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISUBSTITUTINGPROTOCOLHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISubstitutingProtocolHandler;

  enum {
    ALLOW_CONTENT_ACCESS = 1,
    RESOLVE_JAR_URI = 2
  };

  /* [must_use] void setSubstitution (in ACString root, in nsIURI baseURI); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetSubstitution(const nsACString& root, nsIURI *baseURI) = 0;

  /* [must_use] void setSubstitutionWithFlags (in ACString root, in nsIURI baseURI, in uint32_t flags); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SetSubstitutionWithFlags(const nsACString& root, nsIURI *baseURI, uint32_t flags) = 0;

  /* [must_use] nsIURI getSubstitution (in ACString root); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetSubstitution(const nsACString& root, nsIURI **_retval) = 0;

  /* [must_use] boolean hasSubstitution (in ACString root); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD HasSubstitution(const nsACString& root, bool *_retval) = 0;

  /* [must_use] AUTF8String resolveURI (in nsIURI resURI); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD ResolveURI(nsIURI *resURI, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISubstitutingProtocolHandler, NS_ISUBSTITUTINGPROTOCOLHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISUBSTITUTINGPROTOCOLHANDLER \
  [[nodiscard]] NS_IMETHOD SetSubstitution(const nsACString& root, nsIURI *baseURI) override; \
  [[nodiscard]] NS_IMETHOD SetSubstitutionWithFlags(const nsACString& root, nsIURI *baseURI, uint32_t flags) override; \
  [[nodiscard]] NS_IMETHOD GetSubstitution(const nsACString& root, nsIURI **_retval) override; \
  [[nodiscard]] NS_IMETHOD HasSubstitution(const nsACString& root, bool *_retval) override; \
  [[nodiscard]] NS_IMETHOD ResolveURI(nsIURI *resURI, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISUBSTITUTINGPROTOCOLHANDLER \
  [[nodiscard]] nsresult SetSubstitution(const nsACString& root, nsIURI *baseURI); \
  [[nodiscard]] nsresult SetSubstitutionWithFlags(const nsACString& root, nsIURI *baseURI, uint32_t flags); \
  [[nodiscard]] nsresult GetSubstitution(const nsACString& root, nsIURI **_retval); \
  [[nodiscard]] nsresult HasSubstitution(const nsACString& root, bool *_retval); \
  [[nodiscard]] nsresult ResolveURI(nsIURI *resURI, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISUBSTITUTINGPROTOCOLHANDLER(_to) \
  [[nodiscard]] NS_IMETHOD SetSubstitution(const nsACString& root, nsIURI *baseURI) override { return _to SetSubstitution(root, baseURI); } \
  [[nodiscard]] NS_IMETHOD SetSubstitutionWithFlags(const nsACString& root, nsIURI *baseURI, uint32_t flags) override { return _to SetSubstitutionWithFlags(root, baseURI, flags); } \
  [[nodiscard]] NS_IMETHOD GetSubstitution(const nsACString& root, nsIURI **_retval) override { return _to GetSubstitution(root, _retval); } \
  [[nodiscard]] NS_IMETHOD HasSubstitution(const nsACString& root, bool *_retval) override { return _to HasSubstitution(root, _retval); } \
  [[nodiscard]] NS_IMETHOD ResolveURI(nsIURI *resURI, nsACString& _retval) override { return _to ResolveURI(resURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISUBSTITUTINGPROTOCOLHANDLER(_to) \
  [[nodiscard]] NS_IMETHOD SetSubstitution(const nsACString& root, nsIURI *baseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSubstitution(root, baseURI); } \
  [[nodiscard]] NS_IMETHOD SetSubstitutionWithFlags(const nsACString& root, nsIURI *baseURI, uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSubstitutionWithFlags(root, baseURI, flags); } \
  [[nodiscard]] NS_IMETHOD GetSubstitution(const nsACString& root, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubstitution(root, _retval); } \
  [[nodiscard]] NS_IMETHOD HasSubstitution(const nsACString& root, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasSubstitution(root, _retval); } \
  [[nodiscard]] NS_IMETHOD ResolveURI(nsIURI *resURI, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResolveURI(resURI, _retval); } 


#endif /* __gen_nsISubstitutingProtocolHandler_h__ */
