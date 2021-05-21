/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/nsIBrowserHandler.idl
 */

#ifndef __gen_nsIBrowserHandler_h__
#define __gen_nsIBrowserHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICommandLine; /* forward declaration */


/* starting interface:    nsIBrowserHandler */
#define NS_IBROWSERHANDLER_IID_STR "8d3f5a9d-118d-4548-a137-cf7718679069"

#define NS_IBROWSERHANDLER_IID \
  {0x8d3f5a9d, 0x118d, 0x4548, \
    { 0xa1, 0x37, 0xcf, 0x77, 0x18, 0x67, 0x90, 0x69 }}

class NS_NO_VTABLE nsIBrowserHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserHandler;

  /* attribute AUTF8String startPage; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStartPage(nsACString& aStartPage) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStartPage(const nsACString& aStartPage) = 0;

  /* attribute AUTF8String defaultArgs; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultArgs(nsACString& aDefaultArgs) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultArgs(const nsACString& aDefaultArgs) = 0;

  /* attribute boolean kiosk; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKiosk(bool *aKiosk) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetKiosk(bool aKiosk) = 0;

  /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFeatures(nsICommandLine *aCmdLine, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserHandler, NS_IBROWSERHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERHANDLER \
  NS_IMETHOD GetStartPage(nsACString& aStartPage) override; \
  NS_IMETHOD SetStartPage(const nsACString& aStartPage) override; \
  NS_IMETHOD GetDefaultArgs(nsACString& aDefaultArgs) override; \
  NS_IMETHOD SetDefaultArgs(const nsACString& aDefaultArgs) override; \
  NS_IMETHOD GetKiosk(bool *aKiosk) override; \
  NS_IMETHOD SetKiosk(bool aKiosk) override; \
  NS_IMETHOD GetFeatures(nsICommandLine *aCmdLine, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERHANDLER \
  nsresult GetStartPage(nsACString& aStartPage); \
  nsresult SetStartPage(const nsACString& aStartPage); \
  nsresult GetDefaultArgs(nsACString& aDefaultArgs); \
  nsresult SetDefaultArgs(const nsACString& aDefaultArgs); \
  nsresult GetKiosk(bool *aKiosk); \
  nsresult SetKiosk(bool aKiosk); \
  nsresult GetFeatures(nsICommandLine *aCmdLine, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERHANDLER(_to) \
  NS_IMETHOD GetStartPage(nsACString& aStartPage) override { return _to GetStartPage(aStartPage); } \
  NS_IMETHOD SetStartPage(const nsACString& aStartPage) override { return _to SetStartPage(aStartPage); } \
  NS_IMETHOD GetDefaultArgs(nsACString& aDefaultArgs) override { return _to GetDefaultArgs(aDefaultArgs); } \
  NS_IMETHOD SetDefaultArgs(const nsACString& aDefaultArgs) override { return _to SetDefaultArgs(aDefaultArgs); } \
  NS_IMETHOD GetKiosk(bool *aKiosk) override { return _to GetKiosk(aKiosk); } \
  NS_IMETHOD SetKiosk(bool aKiosk) override { return _to SetKiosk(aKiosk); } \
  NS_IMETHOD GetFeatures(nsICommandLine *aCmdLine, nsACString& _retval) override { return _to GetFeatures(aCmdLine, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERHANDLER(_to) \
  NS_IMETHOD GetStartPage(nsACString& aStartPage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartPage(aStartPage); } \
  NS_IMETHOD SetStartPage(const nsACString& aStartPage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStartPage(aStartPage); } \
  NS_IMETHOD GetDefaultArgs(nsACString& aDefaultArgs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultArgs(aDefaultArgs); } \
  NS_IMETHOD SetDefaultArgs(const nsACString& aDefaultArgs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultArgs(aDefaultArgs); } \
  NS_IMETHOD GetKiosk(bool *aKiosk) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKiosk(aKiosk); } \
  NS_IMETHOD SetKiosk(bool aKiosk) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetKiosk(aKiosk); } \
  NS_IMETHOD GetFeatures(nsICommandLine *aCmdLine, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatures(aCmdLine, _retval); } 


#endif /* __gen_nsIBrowserHandler_h__ */
