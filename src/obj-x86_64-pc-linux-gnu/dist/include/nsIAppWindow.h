/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIAppWindow.idl
 */

#ifndef __gen_nsIAppWindow_h__
#define __gen_nsIAppWindow_h__


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
#include "LiveResizeListener.h"
#include "nsTArray.h"
class nsIDocShell; /* forward declaration */

class nsIDocShellTreeItem; /* forward declaration */

class nsIXULBrowserWindow; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIOpenWindowInfo; /* forward declaration */


/* starting interface:    nsIAppWindow */
#define NS_IAPPWINDOW_IID_STR "d6d7a014-e28d-4c9d-8727-1cf6d870619b"

#define NS_IAPPWINDOW_IID \
  {0xd6d7a014, 0xe28d, 0x4c9d, \
    { 0x87, 0x27, 0x1c, 0xf6, 0xd8, 0x70, 0x61, 0x9b }}

class NS_NO_VTABLE nsIAppWindow : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPWINDOW_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAppWindow;

  /* readonly attribute nsIDocShell docShell; */
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) = 0;

  /* attribute boolean intrinsicallySized; */
  NS_IMETHOD GetIntrinsicallySized(bool *aIntrinsicallySized) = 0;
  NS_IMETHOD SetIntrinsicallySized(bool aIntrinsicallySized) = 0;

  /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) = 0;

  /* readonly attribute nsIRemoteTab primaryRemoteTab; */
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) = 0;

  /* void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary); */
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) = 0;

  /* void remoteTabRemoved (in nsIRemoteTab aTab); */
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) = 0;

  /* [noscript,notxpcom] LiveResizeListenerArray getLiveResizeListeners (); */
  NS_IMETHOD_(nsTArray<RefPtr<mozilla::LiveResizeListener>>) GetLiveResizeListeners(void) = 0;

  /* void addChildWindow (in nsIAppWindow aChild); */
  NS_IMETHOD AddChildWindow(nsIAppWindow *aChild) = 0;

  /* [infallible] readonly attribute unsigned long outerToInnerHeightDifferenceInCSSPixels; */
  NS_IMETHOD GetOuterToInnerHeightDifferenceInCSSPixels(uint32_t *aOuterToInnerHeightDifferenceInCSSPixels) = 0;
  inline uint32_t  GetOuterToInnerHeightDifferenceInCSSPixels()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetOuterToInnerHeightDifferenceInCSSPixels(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long outerToInnerWidthDifferenceInCSSPixels; */
  NS_IMETHOD GetOuterToInnerWidthDifferenceInCSSPixels(uint32_t *aOuterToInnerWidthDifferenceInCSSPixels) = 0;
  inline uint32_t  GetOuterToInnerWidthDifferenceInCSSPixels()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetOuterToInnerWidthDifferenceInCSSPixels(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* void removeChildWindow (in nsIAppWindow aChild); */
  NS_IMETHOD RemoveChildWindow(nsIAppWindow *aChild) = 0;

  /* void center (in nsIAppWindow aRelative, in boolean aScreen, in boolean aAlert); */
  NS_IMETHOD Center(nsIAppWindow *aRelative, bool aScreen, bool aAlert) = 0;

  /* void showModal (); */
  NS_IMETHOD ShowModal(void) = 0;

  /* void lockAspectRatio (in bool aShouldLock); */
  NS_IMETHOD LockAspectRatio(bool aShouldLock) = 0;

  enum {
    lowestZ = 0U,
    loweredZ = 4U,
    normalZ = 5U,
    raisedZ = 6U,
    highestZ = 9U
  };

  /* attribute unsigned long zLevel; */
  NS_IMETHOD GetZLevel(uint32_t *aZLevel) = 0;
  NS_IMETHOD SetZLevel(uint32_t aZLevel) = 0;

  /* attribute uint32_t chromeFlags; */
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) = 0;
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) = 0;

  /* void assumeChromeFlagsAreFrozen (); */
  NS_IMETHOD AssumeChromeFlagsAreFrozen(void) = 0;

  /* nsIAppWindow createNewWindow (in int32_t aChromeFlags, in nsIOpenWindowInfo aOpenWindowInfo); */
  NS_IMETHOD CreateNewWindow(int32_t aChromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, nsIAppWindow **_retval) = 0;

  /* attribute nsIXULBrowserWindow XULBrowserWindow; */
  NS_IMETHOD GetXULBrowserWindow(nsIXULBrowserWindow **aXULBrowserWindow) = 0;
  NS_IMETHOD SetXULBrowserWindow(nsIXULBrowserWindow *aXULBrowserWindow) = 0;

  /* [noscript] void beforeStartLayout (); */
  NS_IMETHOD BeforeStartLayout(void) = 0;

  /* [noscript,notxpcom] void sizeShellToWithLimit (in int32_t aDesiredWidth, in int32_t aDesiredHeight, in int32_t shellItemWidth, in int32_t shellItemHeight); */
  NS_IMETHOD_(void) SizeShellToWithLimit(int32_t aDesiredWidth, int32_t aDesiredHeight, int32_t shellItemWidth, int32_t shellItemHeight) = 0;

  /* readonly attribute nsIOpenWindowInfo initialOpenWindowInfo; */
  NS_IMETHOD GetInitialOpenWindowInfo(nsIOpenWindowInfo **aInitialOpenWindowInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAppWindow, NS_IAPPWINDOW_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPWINDOW \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override; \
  NS_IMETHOD GetIntrinsicallySized(bool *aIntrinsicallySized) override; \
  NS_IMETHOD SetIntrinsicallySized(bool aIntrinsicallySized) override; \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override; \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override; \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override; \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override; \
  NS_IMETHOD_(nsTArray<RefPtr<mozilla::LiveResizeListener>>) GetLiveResizeListeners(void) override; \
  NS_IMETHOD AddChildWindow(nsIAppWindow *aChild) override; \
  using nsIAppWindow::GetOuterToInnerHeightDifferenceInCSSPixels; \
  NS_IMETHOD GetOuterToInnerHeightDifferenceInCSSPixels(uint32_t *aOuterToInnerHeightDifferenceInCSSPixels) override; \
  using nsIAppWindow::GetOuterToInnerWidthDifferenceInCSSPixels; \
  NS_IMETHOD GetOuterToInnerWidthDifferenceInCSSPixels(uint32_t *aOuterToInnerWidthDifferenceInCSSPixels) override; \
  NS_IMETHOD RemoveChildWindow(nsIAppWindow *aChild) override; \
  NS_IMETHOD Center(nsIAppWindow *aRelative, bool aScreen, bool aAlert) override; \
  NS_IMETHOD ShowModal(void) override; \
  NS_IMETHOD LockAspectRatio(bool aShouldLock) override; \
  NS_IMETHOD GetZLevel(uint32_t *aZLevel) override; \
  NS_IMETHOD SetZLevel(uint32_t aZLevel) override; \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override; \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override; \
  NS_IMETHOD AssumeChromeFlagsAreFrozen(void) override; \
  NS_IMETHOD CreateNewWindow(int32_t aChromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, nsIAppWindow **_retval) override; \
  NS_IMETHOD GetXULBrowserWindow(nsIXULBrowserWindow **aXULBrowserWindow) override; \
  NS_IMETHOD SetXULBrowserWindow(nsIXULBrowserWindow *aXULBrowserWindow) override; \
  NS_IMETHOD BeforeStartLayout(void) override; \
  NS_IMETHOD_(void) SizeShellToWithLimit(int32_t aDesiredWidth, int32_t aDesiredHeight, int32_t shellItemWidth, int32_t shellItemHeight) override; \
  NS_IMETHOD GetInitialOpenWindowInfo(nsIOpenWindowInfo **aInitialOpenWindowInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPWINDOW \
  nsresult GetDocShell(nsIDocShell **aDocShell); \
  nsresult GetIntrinsicallySized(bool *aIntrinsicallySized); \
  nsresult SetIntrinsicallySized(bool aIntrinsicallySized); \
  nsresult GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell); \
  nsresult GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab); \
  nsresult RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary); \
  nsresult RemoteTabRemoved(nsIRemoteTab *aTab); \
  nsresult_(nsTArray<RefPtr<mozilla::LiveResizeListener>>) GetLiveResizeListeners(void); \
  nsresult AddChildWindow(nsIAppWindow *aChild); \
  using nsIAppWindow::GetOuterToInnerHeightDifferenceInCSSPixels; \
  nsresult GetOuterToInnerHeightDifferenceInCSSPixels(uint32_t *aOuterToInnerHeightDifferenceInCSSPixels); \
  using nsIAppWindow::GetOuterToInnerWidthDifferenceInCSSPixels; \
  nsresult GetOuterToInnerWidthDifferenceInCSSPixels(uint32_t *aOuterToInnerWidthDifferenceInCSSPixels); \
  nsresult RemoveChildWindow(nsIAppWindow *aChild); \
  nsresult Center(nsIAppWindow *aRelative, bool aScreen, bool aAlert); \
  nsresult ShowModal(void); \
  nsresult LockAspectRatio(bool aShouldLock); \
  nsresult GetZLevel(uint32_t *aZLevel); \
  nsresult SetZLevel(uint32_t aZLevel); \
  nsresult GetChromeFlags(uint32_t *aChromeFlags); \
  nsresult SetChromeFlags(uint32_t aChromeFlags); \
  nsresult AssumeChromeFlagsAreFrozen(void); \
  nsresult CreateNewWindow(int32_t aChromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, nsIAppWindow **_retval); \
  nsresult GetXULBrowserWindow(nsIXULBrowserWindow **aXULBrowserWindow); \
  nsresult SetXULBrowserWindow(nsIXULBrowserWindow *aXULBrowserWindow); \
  nsresult BeforeStartLayout(void); \
  nsresult_(void) SizeShellToWithLimit(int32_t aDesiredWidth, int32_t aDesiredHeight, int32_t shellItemWidth, int32_t shellItemHeight); \
  nsresult GetInitialOpenWindowInfo(nsIOpenWindowInfo **aInitialOpenWindowInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPWINDOW(_to) \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override { return _to GetDocShell(aDocShell); } \
  NS_IMETHOD GetIntrinsicallySized(bool *aIntrinsicallySized) override { return _to GetIntrinsicallySized(aIntrinsicallySized); } \
  NS_IMETHOD SetIntrinsicallySized(bool aIntrinsicallySized) override { return _to SetIntrinsicallySized(aIntrinsicallySized); } \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override { return _to GetPrimaryContentShell(aPrimaryContentShell); } \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override { return _to GetPrimaryRemoteTab(aPrimaryRemoteTab); } \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override { return _to RemoteTabAdded(aTab, aPrimary); } \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override { return _to RemoteTabRemoved(aTab); } \
  NS_IMETHOD_(nsTArray<RefPtr<mozilla::LiveResizeListener>>) GetLiveResizeListeners(void) override { return _to GetLiveResizeListeners(); } \
  NS_IMETHOD AddChildWindow(nsIAppWindow *aChild) override { return _to AddChildWindow(aChild); } \
  using nsIAppWindow::GetOuterToInnerHeightDifferenceInCSSPixels; \
  NS_IMETHOD GetOuterToInnerHeightDifferenceInCSSPixels(uint32_t *aOuterToInnerHeightDifferenceInCSSPixels) override { return _to GetOuterToInnerHeightDifferenceInCSSPixels(aOuterToInnerHeightDifferenceInCSSPixels); } \
  using nsIAppWindow::GetOuterToInnerWidthDifferenceInCSSPixels; \
  NS_IMETHOD GetOuterToInnerWidthDifferenceInCSSPixels(uint32_t *aOuterToInnerWidthDifferenceInCSSPixels) override { return _to GetOuterToInnerWidthDifferenceInCSSPixels(aOuterToInnerWidthDifferenceInCSSPixels); } \
  NS_IMETHOD RemoveChildWindow(nsIAppWindow *aChild) override { return _to RemoveChildWindow(aChild); } \
  NS_IMETHOD Center(nsIAppWindow *aRelative, bool aScreen, bool aAlert) override { return _to Center(aRelative, aScreen, aAlert); } \
  NS_IMETHOD ShowModal(void) override { return _to ShowModal(); } \
  NS_IMETHOD LockAspectRatio(bool aShouldLock) override { return _to LockAspectRatio(aShouldLock); } \
  NS_IMETHOD GetZLevel(uint32_t *aZLevel) override { return _to GetZLevel(aZLevel); } \
  NS_IMETHOD SetZLevel(uint32_t aZLevel) override { return _to SetZLevel(aZLevel); } \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override { return _to GetChromeFlags(aChromeFlags); } \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override { return _to SetChromeFlags(aChromeFlags); } \
  NS_IMETHOD AssumeChromeFlagsAreFrozen(void) override { return _to AssumeChromeFlagsAreFrozen(); } \
  NS_IMETHOD CreateNewWindow(int32_t aChromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, nsIAppWindow **_retval) override { return _to CreateNewWindow(aChromeFlags, aOpenWindowInfo, _retval); } \
  NS_IMETHOD GetXULBrowserWindow(nsIXULBrowserWindow **aXULBrowserWindow) override { return _to GetXULBrowserWindow(aXULBrowserWindow); } \
  NS_IMETHOD SetXULBrowserWindow(nsIXULBrowserWindow *aXULBrowserWindow) override { return _to SetXULBrowserWindow(aXULBrowserWindow); } \
  NS_IMETHOD BeforeStartLayout(void) override { return _to BeforeStartLayout(); } \
  NS_IMETHOD_(void) SizeShellToWithLimit(int32_t aDesiredWidth, int32_t aDesiredHeight, int32_t shellItemWidth, int32_t shellItemHeight) override { return _to SizeShellToWithLimit(aDesiredWidth, aDesiredHeight, shellItemWidth, shellItemHeight); } \
  NS_IMETHOD GetInitialOpenWindowInfo(nsIOpenWindowInfo **aInitialOpenWindowInfo) override { return _to GetInitialOpenWindowInfo(aInitialOpenWindowInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPWINDOW(_to) \
  NS_IMETHOD GetDocShell(nsIDocShell **aDocShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocShell(aDocShell); } \
  NS_IMETHOD GetIntrinsicallySized(bool *aIntrinsicallySized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIntrinsicallySized(aIntrinsicallySized); } \
  NS_IMETHOD SetIntrinsicallySized(bool aIntrinsicallySized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIntrinsicallySized(aIntrinsicallySized); } \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryContentShell(aPrimaryContentShell); } \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryRemoteTab(aPrimaryRemoteTab); } \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteTabAdded(aTab, aPrimary); } \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteTabRemoved(aTab); } \
  NS_IMETHOD_(nsTArray<RefPtr<mozilla::LiveResizeListener>>) GetLiveResizeListeners(void) override; \
  NS_IMETHOD AddChildWindow(nsIAppWindow *aChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddChildWindow(aChild); } \
  NS_IMETHOD GetOuterToInnerHeightDifferenceInCSSPixels(uint32_t *aOuterToInnerHeightDifferenceInCSSPixels) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOuterToInnerHeightDifferenceInCSSPixels(aOuterToInnerHeightDifferenceInCSSPixels); } \
  NS_IMETHOD GetOuterToInnerWidthDifferenceInCSSPixels(uint32_t *aOuterToInnerWidthDifferenceInCSSPixels) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOuterToInnerWidthDifferenceInCSSPixels(aOuterToInnerWidthDifferenceInCSSPixels); } \
  NS_IMETHOD RemoveChildWindow(nsIAppWindow *aChild) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveChildWindow(aChild); } \
  NS_IMETHOD Center(nsIAppWindow *aRelative, bool aScreen, bool aAlert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Center(aRelative, aScreen, aAlert); } \
  NS_IMETHOD ShowModal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ShowModal(); } \
  NS_IMETHOD LockAspectRatio(bool aShouldLock) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LockAspectRatio(aShouldLock); } \
  NS_IMETHOD GetZLevel(uint32_t *aZLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetZLevel(aZLevel); } \
  NS_IMETHOD SetZLevel(uint32_t aZLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetZLevel(aZLevel); } \
  NS_IMETHOD GetChromeFlags(uint32_t *aChromeFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeFlags(aChromeFlags); } \
  NS_IMETHOD SetChromeFlags(uint32_t aChromeFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetChromeFlags(aChromeFlags); } \
  NS_IMETHOD AssumeChromeFlagsAreFrozen(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AssumeChromeFlagsAreFrozen(); } \
  NS_IMETHOD CreateNewWindow(int32_t aChromeFlags, nsIOpenWindowInfo *aOpenWindowInfo, nsIAppWindow **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateNewWindow(aChromeFlags, aOpenWindowInfo, _retval); } \
  NS_IMETHOD GetXULBrowserWindow(nsIXULBrowserWindow **aXULBrowserWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetXULBrowserWindow(aXULBrowserWindow); } \
  NS_IMETHOD SetXULBrowserWindow(nsIXULBrowserWindow *aXULBrowserWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetXULBrowserWindow(aXULBrowserWindow); } \
  NS_IMETHOD BeforeStartLayout(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeforeStartLayout(); } \
  NS_IMETHOD_(void) SizeShellToWithLimit(int32_t aDesiredWidth, int32_t aDesiredHeight, int32_t shellItemWidth, int32_t shellItemHeight) override; \
  NS_IMETHOD GetInitialOpenWindowInfo(nsIOpenWindowInfo **aInitialOpenWindowInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitialOpenWindowInfo(aInitialOpenWindowInfo); } 


#endif /* __gen_nsIAppWindow_h__ */
