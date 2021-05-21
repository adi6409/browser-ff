/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIOpenWindowInfo.idl
 */

#ifndef __gen_nsIOpenWindowInfo_h__
#define __gen_nsIOpenWindowInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

namespace mozilla {
class OriginAttributes;
namespace dom {
class BrowserParent;
} // namespace dom
} // namespace mozilla

/* starting interface:    nsIBrowsingContextReadyCallback */
#define NS_IBROWSINGCONTEXTREADYCALLBACK_IID_STR "0524ee06-7f4c-4cd3-ab80-084562745cad"

#define NS_IBROWSINGCONTEXTREADYCALLBACK_IID \
  {0x0524ee06, 0x7f4c, 0x4cd3, \
    { 0xab, 0x80, 0x08, 0x45, 0x62, 0x74, 0x5c, 0xad }}

class NS_NO_VTABLE nsIBrowsingContextReadyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSINGCONTEXTREADYCALLBACK_IID)

  /* void browsingContextReady (in BrowsingContext bc); */
  NS_IMETHOD BrowsingContextReady(mozilla::dom::BrowsingContext *bc) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowsingContextReadyCallback, NS_IBROWSINGCONTEXTREADYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSINGCONTEXTREADYCALLBACK \
  NS_IMETHOD BrowsingContextReady(mozilla::dom::BrowsingContext *bc) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSINGCONTEXTREADYCALLBACK \
  nsresult BrowsingContextReady(mozilla::dom::BrowsingContext *bc); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSINGCONTEXTREADYCALLBACK(_to) \
  NS_IMETHOD BrowsingContextReady(mozilla::dom::BrowsingContext *bc) override { return _to BrowsingContextReady(bc); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSINGCONTEXTREADYCALLBACK(_to) \
  NS_IMETHOD BrowsingContextReady(mozilla::dom::BrowsingContext *bc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BrowsingContextReady(bc); } 


/* starting interface:    nsIOpenWindowInfo */
#define NS_IOPENWINDOWINFO_IID_STR "30359edb-126c-4f65-ae80-07fb158697f9"

#define NS_IOPENWINDOWINFO_IID \
  {0x30359edb, 0x126c, 0x4f65, \
    { 0xae, 0x80, 0x07, 0xfb, 0x15, 0x86, 0x97, 0xf9 }}

class NS_NO_VTABLE nsIOpenWindowInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IOPENWINDOWINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIOpenWindowInfo;

  /* [infallible] readonly attribute BrowsingContext parent; */
  NS_IMETHOD GetParent(mozilla::dom::BrowsingContext **aParent) = 0;
   inline already_AddRefed<mozilla::dom::BrowsingContext> GetParent()
  {
    mozilla::dom::BrowsingContext* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetParent(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::BrowsingContext>(result);
  }

  /* [infallible] readonly attribute boolean isRemote; */
  NS_IMETHOD GetIsRemote(bool *aIsRemote) = 0;
  inline bool  GetIsRemote()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsRemote(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean forceNoOpener; */
  NS_IMETHOD GetForceNoOpener(bool *aForceNoOpener) = 0;
  inline bool  GetForceNoOpener()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForceNoOpener(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isForPrinting; */
  NS_IMETHOD GetIsForPrinting(bool *aIsForPrinting) = 0;
  inline bool  GetIsForPrinting()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsForPrinting(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isForWindowDotPrint; */
  NS_IMETHOD GetIsForWindowDotPrint(bool *aIsForWindowDotPrint) = 0;
  inline bool  GetIsForWindowDotPrint()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsForWindowDotPrint(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [nostdcall,notxpcom] BrowserParent getNextRemoteBrowser (); */
  virtual mozilla::dom::BrowserParent * GetNextRemoteBrowser(void) = 0;

  /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes; */
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  /* [binaryname(GetOriginAttributes),nostdcall,notxpcom] const_OriginAttributes binaryGetOriginAttributes (); */
  virtual const mozilla::OriginAttributes & GetOriginAttributes(void) = 0;

  /* [nostdcall,notxpcom] nsIBrowsingContextReadyCallback browsingContextReadyCallback (); */
  virtual nsIBrowsingContextReadyCallback * BrowsingContextReadyCallback(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIOpenWindowInfo, NS_IOPENWINDOWINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIOPENWINDOWINFO \
  using nsIOpenWindowInfo::GetParent; \
  NS_IMETHOD GetParent(mozilla::dom::BrowsingContext **aParent) override; \
  using nsIOpenWindowInfo::GetIsRemote; \
  NS_IMETHOD GetIsRemote(bool *aIsRemote) override; \
  using nsIOpenWindowInfo::GetForceNoOpener; \
  NS_IMETHOD GetForceNoOpener(bool *aForceNoOpener) override; \
  using nsIOpenWindowInfo::GetIsForPrinting; \
  NS_IMETHOD GetIsForPrinting(bool *aIsForPrinting) override; \
  using nsIOpenWindowInfo::GetIsForWindowDotPrint; \
  NS_IMETHOD GetIsForWindowDotPrint(bool *aIsForWindowDotPrint) override; \
  virtual mozilla::dom::BrowserParent * GetNextRemoteBrowser(void) override; \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  virtual const mozilla::OriginAttributes & GetOriginAttributes(void) override; \
  virtual nsIBrowsingContextReadyCallback * BrowsingContextReadyCallback(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIOPENWINDOWINFO \
  using nsIOpenWindowInfo::GetParent; \
  nsresult GetParent(mozilla::dom::BrowsingContext **aParent); \
  using nsIOpenWindowInfo::GetIsRemote; \
  nsresult GetIsRemote(bool *aIsRemote); \
  using nsIOpenWindowInfo::GetForceNoOpener; \
  nsresult GetForceNoOpener(bool *aForceNoOpener); \
  using nsIOpenWindowInfo::GetIsForPrinting; \
  nsresult GetIsForPrinting(bool *aIsForPrinting); \
  using nsIOpenWindowInfo::GetIsForWindowDotPrint; \
  nsresult GetIsForWindowDotPrint(bool *aIsForWindowDotPrint); \
  mozilla::dom::BrowserParent * GetNextRemoteBrowser(void); \
  nsresult GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  const mozilla::OriginAttributes & GetOriginAttributes(void); \
  nsIBrowsingContextReadyCallback * BrowsingContextReadyCallback(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIOPENWINDOWINFO(_to) \
  using nsIOpenWindowInfo::GetParent; \
  NS_IMETHOD GetParent(mozilla::dom::BrowsingContext **aParent) override { return _to GetParent(aParent); } \
  using nsIOpenWindowInfo::GetIsRemote; \
  NS_IMETHOD GetIsRemote(bool *aIsRemote) override { return _to GetIsRemote(aIsRemote); } \
  using nsIOpenWindowInfo::GetForceNoOpener; \
  NS_IMETHOD GetForceNoOpener(bool *aForceNoOpener) override { return _to GetForceNoOpener(aForceNoOpener); } \
  using nsIOpenWindowInfo::GetIsForPrinting; \
  NS_IMETHOD GetIsForPrinting(bool *aIsForPrinting) override { return _to GetIsForPrinting(aIsForPrinting); } \
  using nsIOpenWindowInfo::GetIsForWindowDotPrint; \
  NS_IMETHOD GetIsForWindowDotPrint(bool *aIsForWindowDotPrint) override { return _to GetIsForWindowDotPrint(aIsForWindowDotPrint); } \
  virtual mozilla::dom::BrowserParent * GetNextRemoteBrowser(void) override { return _to GetNextRemoteBrowser(); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes & GetOriginAttributes(void) override { return _to GetOriginAttributes(); } \
  virtual nsIBrowsingContextReadyCallback * BrowsingContextReadyCallback(void) override { return _to BrowsingContextReadyCallback(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIOPENWINDOWINFO(_to) \
  NS_IMETHOD GetParent(mozilla::dom::BrowsingContext **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParent(aParent); } \
  NS_IMETHOD GetIsRemote(bool *aIsRemote) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsRemote(aIsRemote); } \
  NS_IMETHOD GetForceNoOpener(bool *aForceNoOpener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceNoOpener(aForceNoOpener); } \
  NS_IMETHOD GetIsForPrinting(bool *aIsForPrinting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsForPrinting(aIsForPrinting); } \
  NS_IMETHOD GetIsForWindowDotPrint(bool *aIsForWindowDotPrint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsForWindowDotPrint(aIsForWindowDotPrint); } \
  virtual mozilla::dom::BrowserParent * GetNextRemoteBrowser(void) override; \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes & GetOriginAttributes(void) override; \
  virtual nsIBrowsingContextReadyCallback * BrowsingContextReadyCallback(void) override; 


#endif /* __gen_nsIOpenWindowInfo_h__ */
