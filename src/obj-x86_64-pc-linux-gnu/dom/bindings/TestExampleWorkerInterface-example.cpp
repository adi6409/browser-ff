/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "mozilla/dom/TestExampleGenBinding.h"
#include "mozilla/dom/TestExampleWorkerInterface.h"

namespace mozilla {
namespace dom {


// Only needed for refcounted objects.
NS_IMPL_CYCLE_COLLECTION_WRAPPERCACHE_0(TestExampleWorkerInterface)
NS_IMPL_CYCLE_COLLECTING_ADDREF(TestExampleWorkerInterface)
NS_IMPL_CYCLE_COLLECTING_RELEASE(TestExampleWorkerInterface)
NS_INTERFACE_MAP_BEGIN_CYCLE_COLLECTION(TestExampleWorkerInterface)
  NS_WRAPPERCACHE_INTERFACE_MAP_ENTRY
  NS_INTERFACE_MAP_ENTRY(nsISupports)
NS_INTERFACE_MAP_END

TestExampleWorkerInterface::TestExampleWorkerInterface()
{
    // Add |MOZ_COUNT_CTOR(TestExampleWorkerInterface);| for a non-refcounted object.
}

TestExampleWorkerInterface::~TestExampleWorkerInterface()
{
    // Add |MOZ_COUNT_DTOR(TestExampleWorkerInterface);| for a non-refcounted object.
}

JSObject*
TestExampleWorkerInterface::WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto)
{
  return TestExampleWorkerInterface_Binding::Wrap(aCx, this, aGivenProto);
}


} // namespace dom
} // namespace mozilla
