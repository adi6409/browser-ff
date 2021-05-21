/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIDOMMozBrowserFrame.idl
 */

#ifndef __gen_nsIDOMMozBrowserFrame_h__
#define __gen_nsIDOMMozBrowserFrame_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDOMMozBrowserFrame */
#define NS_IDOMMOZBROWSERFRAME_IID_STR "4cafe116-581b-4194-b0de-7f02378fc51d"

#define NS_IDOMMOZBROWSERFRAME_IID \
  {0x4cafe116, 0x581b, 0x4194, \
    { 0xb0, 0xde, 0x7f, 0x02, 0x37, 0x8f, 0xc5, 0x1d }}

class NS_NO_VTABLE nsIDOMMozBrowserFrame : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMMOZBROWSERFRAME_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMMozBrowserFrame;

  /* [infallible] attribute boolean mozbrowser; */
  NS_IMETHOD GetMozbrowser(bool *aMozbrowser) = 0;
  inline bool  GetMozbrowser()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetMozbrowser(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetMozbrowser(bool aMozbrowser) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMMozBrowserFrame, NS_IDOMMOZBROWSERFRAME_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMMOZBROWSERFRAME \
  using nsIDOMMozBrowserFrame::GetMozbrowser; \
  NS_IMETHOD GetMozbrowser(bool *aMozbrowser) override; \
  NS_IMETHOD SetMozbrowser(bool aMozbrowser) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMMOZBROWSERFRAME \
  using nsIDOMMozBrowserFrame::GetMozbrowser; \
  nsresult GetMozbrowser(bool *aMozbrowser); \
  nsresult SetMozbrowser(bool aMozbrowser); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMMOZBROWSERFRAME(_to) \
  using nsIDOMMozBrowserFrame::GetMozbrowser; \
  NS_IMETHOD GetMozbrowser(bool *aMozbrowser) override { return _to GetMozbrowser(aMozbrowser); } \
  NS_IMETHOD SetMozbrowser(bool aMozbrowser) override { return _to SetMozbrowser(aMozbrowser); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMMOZBROWSERFRAME(_to) \
  NS_IMETHOD GetMozbrowser(bool *aMozbrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMozbrowser(aMozbrowser); } \
  NS_IMETHOD SetMozbrowser(bool aMozbrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMozbrowser(aMozbrowser); } 


#endif /* __gen_nsIDOMMozBrowserFrame_h__ */
