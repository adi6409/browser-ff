/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShellTreeOwner.idl
 */

#ifndef __gen_nsIDocShellTreeOwner_h__
#define __gen_nsIDocShellTreeOwner_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDocShellTreeItem; /* forward declaration */

class nsIRemoteTab; /* forward declaration */


/* starting interface:    nsIDocShellTreeOwner */
#define NS_IDOCSHELLTREEOWNER_IID_STR "0e3dc4b1-4cea-4a37-af71-79f0afd07574"

#define NS_IDOCSHELLTREEOWNER_IID \
  {0x0e3dc4b1, 0x4cea, 0x4a37, \
    { 0xaf, 0x71, 0x79, 0xf0, 0xaf, 0xd0, 0x75, 0x74 }}

class NS_NO_VTABLE nsIDocShellTreeOwner : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCSHELLTREEOWNER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocShellTreeOwner;

  /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ContentShellAdded(nsIDocShellTreeItem *aContentShell, bool aPrimary) = 0;

  /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ContentShellRemoved(nsIDocShellTreeItem *aContentShell) = 0;

  /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) = 0;

  /* void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) = 0;

  /* void remoteTabRemoved (in nsIRemoteTab aTab); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) = 0;

  /* readonly attribute nsIRemoteTab primaryRemoteTab; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) = 0;

  /* [can_run_script] void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD SizeShellTo(nsIDocShellTreeItem *shell, int32_t cx, int32_t cy) = 0;

  /* void getPrimaryContentSize (out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryContentSize(int32_t *width, int32_t *height) = 0;

  /* void setPrimaryContentSize (in long width, in long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrimaryContentSize(int32_t width, int32_t height) = 0;

  /* void getRootShellSize (out long width, out long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRootShellSize(int32_t *width, int32_t *height) = 0;

  /* void setRootShellSize (in long width, in long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRootShellSize(int32_t width, int32_t height) = 0;

  /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPersistence(bool aPersistPosition, bool aPersistSize, bool aPersistSizeMode) = 0;

  /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPersistence(bool *aPersistPosition, bool *aPersistSize, bool *aPersistSizeMode) = 0;

  /* readonly attribute unsigned long tabCount; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTabCount(uint32_t *aTabCount) = 0;

  /* readonly attribute bool hasPrimaryContent; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasPrimaryContent(bool *aHasPrimaryContent) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocShellTreeOwner, NS_IDOCSHELLTREEOWNER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCSHELLTREEOWNER \
  NS_IMETHOD ContentShellAdded(nsIDocShellTreeItem *aContentShell, bool aPrimary) override; \
  NS_IMETHOD ContentShellRemoved(nsIDocShellTreeItem *aContentShell) override; \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override; \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override; \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override; \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override; \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SizeShellTo(nsIDocShellTreeItem *shell, int32_t cx, int32_t cy) override; \
  NS_IMETHOD GetPrimaryContentSize(int32_t *width, int32_t *height) override; \
  NS_IMETHOD SetPrimaryContentSize(int32_t width, int32_t height) override; \
  NS_IMETHOD GetRootShellSize(int32_t *width, int32_t *height) override; \
  NS_IMETHOD SetRootShellSize(int32_t width, int32_t height) override; \
  NS_IMETHOD SetPersistence(bool aPersistPosition, bool aPersistSize, bool aPersistSizeMode) override; \
  NS_IMETHOD GetPersistence(bool *aPersistPosition, bool *aPersistSize, bool *aPersistSizeMode) override; \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override; \
  NS_IMETHOD GetHasPrimaryContent(bool *aHasPrimaryContent) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCSHELLTREEOWNER \
  nsresult ContentShellAdded(nsIDocShellTreeItem *aContentShell, bool aPrimary); \
  nsresult ContentShellRemoved(nsIDocShellTreeItem *aContentShell); \
  nsresult GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell); \
  nsresult RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary); \
  nsresult RemoteTabRemoved(nsIRemoteTab *aTab); \
  nsresult GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab); \
  MOZ_CAN_RUN_SCRIPT nsresult SizeShellTo(nsIDocShellTreeItem *shell, int32_t cx, int32_t cy); \
  nsresult GetPrimaryContentSize(int32_t *width, int32_t *height); \
  nsresult SetPrimaryContentSize(int32_t width, int32_t height); \
  nsresult GetRootShellSize(int32_t *width, int32_t *height); \
  nsresult SetRootShellSize(int32_t width, int32_t height); \
  nsresult SetPersistence(bool aPersistPosition, bool aPersistSize, bool aPersistSizeMode); \
  nsresult GetPersistence(bool *aPersistPosition, bool *aPersistSize, bool *aPersistSizeMode); \
  nsresult GetTabCount(uint32_t *aTabCount); \
  nsresult GetHasPrimaryContent(bool *aHasPrimaryContent); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCSHELLTREEOWNER(_to) \
  NS_IMETHOD ContentShellAdded(nsIDocShellTreeItem *aContentShell, bool aPrimary) override { return _to ContentShellAdded(aContentShell, aPrimary); } \
  NS_IMETHOD ContentShellRemoved(nsIDocShellTreeItem *aContentShell) override { return _to ContentShellRemoved(aContentShell); } \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override { return _to GetPrimaryContentShell(aPrimaryContentShell); } \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override { return _to RemoteTabAdded(aTab, aPrimary); } \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override { return _to RemoteTabRemoved(aTab); } \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override { return _to GetPrimaryRemoteTab(aPrimaryRemoteTab); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SizeShellTo(nsIDocShellTreeItem *shell, int32_t cx, int32_t cy) override { return _to SizeShellTo(shell, cx, cy); } \
  NS_IMETHOD GetPrimaryContentSize(int32_t *width, int32_t *height) override { return _to GetPrimaryContentSize(width, height); } \
  NS_IMETHOD SetPrimaryContentSize(int32_t width, int32_t height) override { return _to SetPrimaryContentSize(width, height); } \
  NS_IMETHOD GetRootShellSize(int32_t *width, int32_t *height) override { return _to GetRootShellSize(width, height); } \
  NS_IMETHOD SetRootShellSize(int32_t width, int32_t height) override { return _to SetRootShellSize(width, height); } \
  NS_IMETHOD SetPersistence(bool aPersistPosition, bool aPersistSize, bool aPersistSizeMode) override { return _to SetPersistence(aPersistPosition, aPersistSize, aPersistSizeMode); } \
  NS_IMETHOD GetPersistence(bool *aPersistPosition, bool *aPersistSize, bool *aPersistSizeMode) override { return _to GetPersistence(aPersistPosition, aPersistSize, aPersistSizeMode); } \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override { return _to GetTabCount(aTabCount); } \
  NS_IMETHOD GetHasPrimaryContent(bool *aHasPrimaryContent) override { return _to GetHasPrimaryContent(aHasPrimaryContent); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCSHELLTREEOWNER(_to) \
  NS_IMETHOD ContentShellAdded(nsIDocShellTreeItem *aContentShell, bool aPrimary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContentShellAdded(aContentShell, aPrimary); } \
  NS_IMETHOD ContentShellRemoved(nsIDocShellTreeItem *aContentShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContentShellRemoved(aContentShell); } \
  NS_IMETHOD GetPrimaryContentShell(nsIDocShellTreeItem **aPrimaryContentShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryContentShell(aPrimaryContentShell); } \
  NS_IMETHOD RemoteTabAdded(nsIRemoteTab *aTab, bool aPrimary) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteTabAdded(aTab, aPrimary); } \
  NS_IMETHOD RemoteTabRemoved(nsIRemoteTab *aTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoteTabRemoved(aTab); } \
  NS_IMETHOD GetPrimaryRemoteTab(nsIRemoteTab **aPrimaryRemoteTab) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryRemoteTab(aPrimaryRemoteTab); } \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD SizeShellTo(nsIDocShellTreeItem *shell, int32_t cx, int32_t cy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SizeShellTo(shell, cx, cy); } \
  NS_IMETHOD GetPrimaryContentSize(int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryContentSize(width, height); } \
  NS_IMETHOD SetPrimaryContentSize(int32_t width, int32_t height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrimaryContentSize(width, height); } \
  NS_IMETHOD GetRootShellSize(int32_t *width, int32_t *height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootShellSize(width, height); } \
  NS_IMETHOD SetRootShellSize(int32_t width, int32_t height) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRootShellSize(width, height); } \
  NS_IMETHOD SetPersistence(bool aPersistPosition, bool aPersistSize, bool aPersistSizeMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPersistence(aPersistPosition, aPersistSize, aPersistSizeMode); } \
  NS_IMETHOD GetPersistence(bool *aPersistPosition, bool *aPersistSize, bool *aPersistSizeMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPersistence(aPersistPosition, aPersistSize, aPersistSizeMode); } \
  NS_IMETHOD GetTabCount(uint32_t *aTabCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTabCount(aTabCount); } \
  NS_IMETHOD GetHasPrimaryContent(bool *aHasPrimaryContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasPrimaryContent(aHasPrimaryContent); } 


#endif /* __gen_nsIDocShellTreeOwner_h__ */
