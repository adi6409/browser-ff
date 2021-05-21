/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsILoadContext.idl
 */

#ifndef __gen_nsILoadContext_h__
#define __gen_nsILoadContext_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

#ifdef MOZILLA_INTERNAL_API
namespace mozilla {
class OriginAttributes;
}
#endif

/* starting interface:    nsILoadContext */
#define NS_ILOADCONTEXT_IID_STR "2813a7a3-d084-4d00-acd0-f76620315c02"

#define NS_ILOADCONTEXT_IID \
  {0x2813a7a3, 0xd084, 0x4d00, \
    { 0xac, 0xd0, 0xf7, 0x66, 0x20, 0x31, 0x5c, 0x02 }}

class nsILoadContext : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADCONTEXT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadContext;

  /* readonly attribute mozIDOMWindowProxy associatedWindow; */
  NS_IMETHOD GetAssociatedWindow(mozIDOMWindowProxy **aAssociatedWindow) = 0;

  /* readonly attribute mozIDOMWindowProxy topWindow; */
  NS_IMETHOD GetTopWindow(mozIDOMWindowProxy **aTopWindow) = 0;

  /* readonly attribute Element topFrameElement; */
  NS_IMETHOD GetTopFrameElement(mozilla::dom::Element **aTopFrameElement) = 0;

  /* readonly attribute boolean isContent; */
  NS_IMETHOD GetIsContent(bool *aIsContent) = 0;

  /* attribute boolean usePrivateBrowsing; */
  NS_IMETHOD GetUsePrivateBrowsing(bool *aUsePrivateBrowsing) = 0;
  NS_IMETHOD SetUsePrivateBrowsing(bool aUsePrivateBrowsing) = 0;

  /* readonly attribute boolean useRemoteTabs; */
  NS_IMETHOD GetUseRemoteTabs(bool *aUseRemoteTabs) = 0;

  /* readonly attribute boolean useRemoteSubframes; */
  NS_IMETHOD GetUseRemoteSubframes(bool *aUseRemoteSubframes) = 0;

  /* attribute boolean useTrackingProtection; */
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) = 0;
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) = 0;

   /**
   * De-XPCOMed getter to make call-sites cleaner.
   */
  bool UsePrivateBrowsing()
  {
    bool usingPB = false;
    GetUsePrivateBrowsing(&usingPB);
    return usingPB;
  }
  bool UseRemoteTabs()
  {
    bool usingRT = false;
    GetUseRemoteTabs(&usingRT);
    return usingRT;
  }
  bool UseRemoteSubframes()
  {
    bool usingRSF = false;
    GetUseRemoteSubframes(&usingRSF);
    return usingRSF;
  }
  bool UseTrackingProtection()
  {
    bool usingTP = false;
    GetUseTrackingProtection(&usingTP);
    return usingTP;
  }
  /* [noscript] void SetPrivateBrowsing (in boolean aInPrivateBrowsing); */
  NS_IMETHOD SetPrivateBrowsing(bool aInPrivateBrowsing) = 0;

  /* [noscript] void SetRemoteTabs (in boolean aUseRemoteTabs); */
  NS_IMETHOD SetRemoteTabs(bool aUseRemoteTabs) = 0;

  /* [noscript] void SetRemoteSubframes (in boolean aUseRemoteSubframes); */
  NS_IMETHOD SetRemoteSubframes(bool aUseRemoteSubframes) = 0;

  /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes; */
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  /* [noscript,notxpcom] void GetOriginAttributes (out OriginAttributes aAttrs); */
  NS_IMETHOD_(void) GetOriginAttributes(mozilla::OriginAttributes & aAttrs) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadContext, NS_ILOADCONTEXT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADCONTEXT \
  NS_IMETHOD GetAssociatedWindow(mozIDOMWindowProxy **aAssociatedWindow) override; \
  NS_IMETHOD GetTopWindow(mozIDOMWindowProxy **aTopWindow) override; \
  NS_IMETHOD GetTopFrameElement(mozilla::dom::Element **aTopFrameElement) override; \
  NS_IMETHOD GetIsContent(bool *aIsContent) override; \
  NS_IMETHOD GetUsePrivateBrowsing(bool *aUsePrivateBrowsing) override; \
  NS_IMETHOD SetUsePrivateBrowsing(bool aUsePrivateBrowsing) override; \
  NS_IMETHOD GetUseRemoteTabs(bool *aUseRemoteTabs) override; \
  NS_IMETHOD GetUseRemoteSubframes(bool *aUseRemoteSubframes) override; \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override; \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override; \
  NS_IMETHOD SetPrivateBrowsing(bool aInPrivateBrowsing) override; \
  NS_IMETHOD SetRemoteTabs(bool aUseRemoteTabs) override; \
  NS_IMETHOD SetRemoteSubframes(bool aUseRemoteSubframes) override; \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  NS_IMETHOD_(void) GetOriginAttributes(mozilla::OriginAttributes & aAttrs) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADCONTEXT \
  nsresult GetAssociatedWindow(mozIDOMWindowProxy **aAssociatedWindow); \
  nsresult GetTopWindow(mozIDOMWindowProxy **aTopWindow); \
  nsresult GetTopFrameElement(mozilla::dom::Element **aTopFrameElement); \
  nsresult GetIsContent(bool *aIsContent); \
  nsresult GetUsePrivateBrowsing(bool *aUsePrivateBrowsing); \
  nsresult SetUsePrivateBrowsing(bool aUsePrivateBrowsing); \
  nsresult GetUseRemoteTabs(bool *aUseRemoteTabs); \
  nsresult GetUseRemoteSubframes(bool *aUseRemoteSubframes); \
  nsresult GetUseTrackingProtection(bool *aUseTrackingProtection); \
  nsresult SetUseTrackingProtection(bool aUseTrackingProtection); \
  nsresult SetPrivateBrowsing(bool aInPrivateBrowsing); \
  nsresult SetRemoteTabs(bool aUseRemoteTabs); \
  nsresult SetRemoteSubframes(bool aUseRemoteSubframes); \
  nsresult GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  nsresult_(void) GetOriginAttributes(mozilla::OriginAttributes & aAttrs); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADCONTEXT(_to) \
  NS_IMETHOD GetAssociatedWindow(mozIDOMWindowProxy **aAssociatedWindow) override { return _to GetAssociatedWindow(aAssociatedWindow); } \
  NS_IMETHOD GetTopWindow(mozIDOMWindowProxy **aTopWindow) override { return _to GetTopWindow(aTopWindow); } \
  NS_IMETHOD GetTopFrameElement(mozilla::dom::Element **aTopFrameElement) override { return _to GetTopFrameElement(aTopFrameElement); } \
  NS_IMETHOD GetIsContent(bool *aIsContent) override { return _to GetIsContent(aIsContent); } \
  NS_IMETHOD GetUsePrivateBrowsing(bool *aUsePrivateBrowsing) override { return _to GetUsePrivateBrowsing(aUsePrivateBrowsing); } \
  NS_IMETHOD SetUsePrivateBrowsing(bool aUsePrivateBrowsing) override { return _to SetUsePrivateBrowsing(aUsePrivateBrowsing); } \
  NS_IMETHOD GetUseRemoteTabs(bool *aUseRemoteTabs) override { return _to GetUseRemoteTabs(aUseRemoteTabs); } \
  NS_IMETHOD GetUseRemoteSubframes(bool *aUseRemoteSubframes) override { return _to GetUseRemoteSubframes(aUseRemoteSubframes); } \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override { return _to GetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override { return _to SetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetPrivateBrowsing(bool aInPrivateBrowsing) override { return _to SetPrivateBrowsing(aInPrivateBrowsing); } \
  NS_IMETHOD SetRemoteTabs(bool aUseRemoteTabs) override { return _to SetRemoteTabs(aUseRemoteTabs); } \
  NS_IMETHOD SetRemoteSubframes(bool aUseRemoteSubframes) override { return _to SetRemoteSubframes(aUseRemoteSubframes); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD_(void) GetOriginAttributes(mozilla::OriginAttributes & aAttrs) override { return _to GetOriginAttributes(aAttrs); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADCONTEXT(_to) \
  NS_IMETHOD GetAssociatedWindow(mozIDOMWindowProxy **aAssociatedWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAssociatedWindow(aAssociatedWindow); } \
  NS_IMETHOD GetTopWindow(mozIDOMWindowProxy **aTopWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopWindow(aTopWindow); } \
  NS_IMETHOD GetTopFrameElement(mozilla::dom::Element **aTopFrameElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopFrameElement(aTopFrameElement); } \
  NS_IMETHOD GetIsContent(bool *aIsContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsContent(aIsContent); } \
  NS_IMETHOD GetUsePrivateBrowsing(bool *aUsePrivateBrowsing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsePrivateBrowsing(aUsePrivateBrowsing); } \
  NS_IMETHOD SetUsePrivateBrowsing(bool aUsePrivateBrowsing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUsePrivateBrowsing(aUsePrivateBrowsing); } \
  NS_IMETHOD GetUseRemoteTabs(bool *aUseRemoteTabs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseRemoteTabs(aUseRemoteTabs); } \
  NS_IMETHOD GetUseRemoteSubframes(bool *aUseRemoteSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseRemoteSubframes(aUseRemoteSubframes); } \
  NS_IMETHOD GetUseTrackingProtection(bool *aUseTrackingProtection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetUseTrackingProtection(bool aUseTrackingProtection) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUseTrackingProtection(aUseTrackingProtection); } \
  NS_IMETHOD SetPrivateBrowsing(bool aInPrivateBrowsing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrivateBrowsing(aInPrivateBrowsing); } \
  NS_IMETHOD SetRemoteTabs(bool aUseRemoteTabs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRemoteTabs(aUseRemoteTabs); } \
  NS_IMETHOD SetRemoteSubframes(bool aUseRemoteSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRemoteSubframes(aUseRemoteSubframes); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD_(void) GetOriginAttributes(mozilla::OriginAttributes & aAttrs) override; 


#endif /* __gen_nsILoadContext_h__ */
