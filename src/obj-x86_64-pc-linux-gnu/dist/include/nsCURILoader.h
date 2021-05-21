/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsCURILoader.idl
 */

#ifndef __gen_nsCURILoader_h__
#define __gen_nsCURILoader_h__


#ifndef __gen_nsIURILoader_h__
#include "nsIURILoader.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_CONTENT_HANDLER_CONTRACTID               "@mozilla.org/uriloader/content-handler;1"
#define NS_CONTENT_HANDLER_CONTRACTID_PREFIX	     NS_CONTENT_HANDLER_CONTRACTID "?type="
/**
 * A category where content listeners can register. The name of the entry must
 * be the content that this listener wants to handle, the value must be a
 * contract ID for the listener. It will be created using createInstance (not
 * getService).
 *
 * Listeners added this way are tried after the initial target of the load and
 * after explicitly registered listeners (nsIURILoader::registerContentListener).
 *
 * These listeners must implement at least nsIURIContentListener (and
 * nsISupports).
 *
 * @see nsICategoryManager
 * @see nsIURIContentListener
 */
#define NS_CONTENT_LISTENER_CATEGORYMANAGER_ENTRY   "external-uricontentlisteners"

#endif /* __gen_nsCURILoader_h__ */
