/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIMozBrowserFrame.idl
 */

#ifndef __gen_nsIMozBrowserFrame_h__
#define __gen_nsIMozBrowserFrame_h__


#ifndef __gen_nsIDOMMozBrowserFrame_h__
#include "nsIDOMMozBrowserFrame.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRemoteTab; /* forward declaration */


/* starting interface:    nsIMozBrowserFrame */
#define NS_IMOZBROWSERFRAME_IID_STR "0c0a862c-1a47-43c0-ae9e-d51835e3e1a6"

#define NS_IMOZBROWSERFRAME_IID \
  {0x0c0a862c, 0x1a47, 0x43c0, \
    { 0xae, 0x9e, 0xd5, 0x18, 0x35, 0xe3, 0xe1, 0xa6 }}

class NS_NO_VTABLE nsIMozBrowserFrame : public nsIDOMMozBrowserFrame {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMOZBROWSERFRAME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMozBrowserFrame;

  /* [infallible] readonly attribute boolean reallyIsBrowser; */
  NS_IMETHOD GetReallyIsBrowser(bool *aReallyIsBrowser) = 0;
  inline bool  GetReallyIsBrowser()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetReallyIsBrowser(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [noscript] void initializeBrowserAPI (); */
  NS_IMETHOD InitializeBrowserAPI(void) = 0;

  /* [noscript] void destroyBrowserFrameScripts (); */
  NS_IMETHOD DestroyBrowserFrameScripts(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMozBrowserFrame, NS_IMOZBROWSERFRAME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMOZBROWSERFRAME \
  using nsIMozBrowserFrame::GetReallyIsBrowser; \
  NS_IMETHOD GetReallyIsBrowser(bool *aReallyIsBrowser) override; \
  NS_IMETHOD InitializeBrowserAPI(void) override; \
  NS_IMETHOD DestroyBrowserFrameScripts(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMOZBROWSERFRAME \
  using nsIMozBrowserFrame::GetReallyIsBrowser; \
  nsresult GetReallyIsBrowser(bool *aReallyIsBrowser); \
  nsresult InitializeBrowserAPI(void); \
  nsresult DestroyBrowserFrameScripts(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMOZBROWSERFRAME(_to) \
  using nsIMozBrowserFrame::GetReallyIsBrowser; \
  NS_IMETHOD GetReallyIsBrowser(bool *aReallyIsBrowser) override { return _to GetReallyIsBrowser(aReallyIsBrowser); } \
  NS_IMETHOD InitializeBrowserAPI(void) override { return _to InitializeBrowserAPI(); } \
  NS_IMETHOD DestroyBrowserFrameScripts(void) override { return _to DestroyBrowserFrameScripts(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMOZBROWSERFRAME(_to) \
  NS_IMETHOD GetReallyIsBrowser(bool *aReallyIsBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReallyIsBrowser(aReallyIsBrowser); } \
  NS_IMETHOD InitializeBrowserAPI(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializeBrowserAPI(); } \
  NS_IMETHOD DestroyBrowserFrameScripts(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DestroyBrowserFrameScripts(); } 


#endif /* __gen_nsIMozBrowserFrame_h__ */
