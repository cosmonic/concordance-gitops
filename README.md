# Concordance Banking Example

[➡️Click me to clone this repo][new-repository-link]

> [!IMPORTANT]
> `cosmonic/concordance-gitops` repository is intended to be used as a [template for you to generate your own version](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template) of the demo project for your own testing.
>
> When you create your own repository from the template, it is important to select the option for `Include all branches`, as that is used as part of the initial re-configuration to setup steps for the demo.
>
> Once you have created the repository, [an automated GitHub Action gets kicked off to do a set of customization](./actions) to replace values in the template README with values that match your newly created repository.
>
> Before you get started with rest of the README, please check that the [initial GitHub Action](./actions) has finished and then refresh this README to see it with the updated values.

This example illustrates all of the core event sourcing building blocks applied to a simplified banking model. In this sample, we have modeled a bank account aggregate, projector, process manager, and an artificial gateway that can be used for testing simulated inter-bank transfers (processes/sagas).

This particular repository is a template, meant to demonstrate a GitOps flow for deploying a [wasmCloud application][wasmcloud-application] to Cosmonic. The application is a simple banking example, which is a fork of the [Concordance Banking Example][concordance-banking-example].

## Prerequisites

In order to try this repository out, you will need:

1. A Kubernetes cluster with access to deploy to. For local testing, we recommend [kind][`kind`].
2. [Kubectl][`kubectl`] with kubeconfig pointing to your cluster from the previous step. Use `kubectl config view` to validate.
3. Argo CD deployed into your cluster. For local testing, we recommend following [Argo's Getting Started guide][argo-getting-started] for quick setup.
4. [`cosmo` cli][cosmo-cli-download] `v0.23.2` or later installed. Use `cosmo --version` to validate.
5. Account on [Cosmonic][cosmonic]. It's free to sign up!

<br />

## Demo

In this demo, we will walk through a set of steps with each step building on the previous one. With that being said, the steps we will walk through are as follows:

1. [Logging into Cosmonic](#login-to-cosmonic)
1. [Connect your Kubernetes cluster to Cosmonic](#connect-cluster-to-cosmonic)
1. [Deploy the initial state (v0.0.0) of the demo application](#deploy-version-0_0_0)
1. [Deploy v0.1.0 release](#deploy-version-0_1_0)
1. [Deploy v0.2.0 release](#deploy-version-0_2_0)
1. [Deploy v0.3.0 release](#deploy-version-0_3_0)

<br />

### Logging into Cosmonic <a name="login-to-cosmonic"></a>

Before you proceed with the rest of this demo, you will need to make sure that you are logged in on the command-line to Cosmonic.

To check whether you are logged in or not, you can run:

```shell
cosmo whoami
```

If you have not yet logged in, you will see the following message, prompting you to do so:

```
$ cosmo whoami
Failed to open user credentials. Have you run `cosmo login`?
```

Once you are logged in, you will be greeted with a message that should look something like the following with the actual values replaced with your account specific values:

```shell
$ cosmo whoami
Current user information:

Constellation IDs: dd722aff-7849-4f72-8f63-ca843999202c
NATS public key: AAYOBUJ5QG5YF3OYYSJERTW23SPWBTUF2AJRW3C4HPPVV5YOOM6LZWSV
```

With your `cosmo` cli now logged in, you are ready to proceed with the rest of the demo!

<br />

### Connecting your Kubernetes cluster to Cosmonic<a name="connect-cluster-to-cosmonic"></a>

In order to begin deploying WebAssembly-based applications to your Kubernetes cluster, you will need to connect your cluster with Cosmonic.

To connect your Kubernetes cluster, we will deploy `cosmo-controller` in to your cluster by running the following:

```
cosmo connect k8s
```

By default without any options provided, this will:

1. Connect to your Cosmonic account, prompting you to login if you have not already.
2. Attempt to establish a connection to locally configured Kubernetes credentials by checking for existence of `KUBECONFIG` environment variable and then `~/.kube/config` file.
3. Configure and deploy [`cosmo-controller`][cosmo-controller] into the Kubernetes cluster using the locally configured credentials.
4. Deploy a set of wasmCloud hosts using the bundled `CosmonicHostConfig` CRD to the specified namespace (or `default` namespace if no namespace is provided).
5. Deploy [Kubernetes Applier Capability Provider][kubernetes-applier-provider] and [Kubernetes Applier Actor][kubernetes-applier-actor] as [wadm-managed applications][wadm] into your constellation, which will help expose any HTTP-based actors as services inside of your Kubernetes cluster

Once `cosmo connect k8s` has run successfully, you should see the following output with the actual values matching your account:

```shell
$ cosmo connect k8s
Successfully connected to constellation [dd722aff-7849-4f72-8f63-ca843999202c]
✅ Successfully connected to k8s cluster
✅ Successfully configured controller
ℹ️ Applying additional host labels:
✅ Successfully started wasmcloud hosts
✅ Wadm manifest deployed. You can check deployment status at https://app.cosmonic.com/constellation/applications/detail/cosmo-connect-127-0-0-1-default?view=manifest
🔗 Kubernetes cluster successfully connected!
🚀 Open Cosmonic (https://app.cosmonic.com) to interact with your Kubernetes hosts.
```

With your Kubernetes cluster successfully connected to Cosmonic, you are ready to proceed to the next step of the demo!

<br />

### Deploy the initial state (`v0.0.0`) of the demo application <a name="deploy-version-0_0_0"></a>

For the next part, you will need to clone down your version of this repository that you have created based on the `cosmonic/concordance-gitops` template.

Once you have cloned the repository to your machine, you will find `setup/argocd-application.yaml` at the root of this repository, which defines a simple [Argo CD directory-type Application][argo-directory-application].

To deploy this application in to your cluster, at the root of this repository run the following command:

```shell
kubectl apply -f setup/argocd-application.yaml -n argocd
```

When you apply the Argo CD Application definition to your cluster, Argo CD will check out the source code from your repository and deploy the [wadm application][wadm] defined at the root of this repository (in the `wadm.yaml` file) to the `default` namespace in your Kubernetes cluster.

To check the on the state of deployed application, you can check on it by running:

```shell
kubectl get app/bank-account -n default
```

Or you can check via Argo CD by running:

```shell
kubectl get applications.argoproj.io/bank-account --namespace argocd
```

Once the application has successfully been deployed, you can navigate to https://app.cosmonic.com/constellation/applications/detail/bank-account?view=manifest to see your application.

Under the **"Wormholes"** section you will find a link that you can click to view the application running live: <a name="wormhole-section"></a>

![bank-account wormhole section][wormhole]

With the application now deployed to your Kubernetes cluster, you are ready to iterate!

<br />

### Deploy `v0.1.0` release <a name="deploy-version-0_1_0"></a>

Now that you have the initial version of the application, it's time to make some changes.

As part of the initialization we've set up for this repository, we've created 3 release branches (named `release/v0.1.0` .. `release/v0.3.0`) that each have a set of changes which build on the previous release (or the initial state in the case of `v0.1.0`) as a way to demonstrate how you can use GitOps and wadm together to roll out different changes throughout your application's lifecycle.

So to get going with the first set of changes, what you will want to do is set up a Pull Request for the `release/v0.1.0` branch against `main`. Here's a link for you to do exactly that:

[https://github.com/cosmonic/concordance-gitops/compare/release/v0.1.0?expand=1](https://github.com/cosmonic/concordance-gitops/compare/release/v0.1.0?expand=1)

Once you merge this Pull Request, Argo will detect that there are changes and push up the changes in the `wadm.yaml` file. Afterwards, you will be able to reload the [Wormhole from before](#wormhole-section) to see the changes. You’ll see two new events, `CreateAccount` and `AccountCreated`, corresponding to the request to create an account and the event indicating that the account creation was successful.

<br />

### Deploy `v0.2.0` release <a name="deploy-version-0_2_0"></a>

To demonstrate further iterations, we also have a `release/v0.2.0` branch that adds the ability to deposit and withdraw funds into / from an account. This adds additional logic into the aggregate and projector microservices which were previously just handling the creation of accounts.

In order to deploy the changes from `release/v0.2.0`, you can create a Pull Request from the following link:

[https://github.com/cosmonic/concordance-gitops/compare/release/v0.2.0?expand=1](https://github.com/cosmonic/concordance-gitops/compare/release/v0.2.0?expand=1)

Once merged, Argo will once again sync the changes in `wadm.yaml` by applying them against the cluster, which in turn will prompt wadm to deploy the changes into the cluster.

As before, you will be able to view these changes by refreshing the [Wormhole from before](#wormhole-section).

<br />

### Deploy `v0.3.0` release <a name="deploy-version-0_3_0"></a>

Finally, we have one set of changes queued up for you to complete the exercise. These changes add the ability to wire funds between bank accounts using a wire transfer process manager.

Release v0.3.0 introduces many new events so that we can wire money with a robust log of events, including exactly when money is committed to be wired and when it finishes transferring with checks along the way to ensure no account goes below the specified minimum balance.

In order to get these changes deployed, create a Pull Request for the `release/v0.3.0` branch with the following link:

[https://github.com/cosmonic/concordance-gitops/compare/release/v0.3.0?expand=1](https://github.com/cosmonic/concordance-gitops/compare/release/v0.3.0?expand=1)

Once merged, Argo will take care of applying the changes to the cluster and wadm will roll out a new version of the application in response.

As before, you can refresh the [Wormhole from before](#wormhole-section) to see the final changes live!

<br />

## End of demo

That concludes the demo. We hope that this demonstration has helped illustrate how you can combine the familiar practice of GitOps with the new and emerging technology of WebAssembly!

If you would like to learn more about how wasmCloud and Cosmonic can help you realize the benefits of WebAssembly in development and production, you can connect with us at:

- [Cosmonic Discord](https://discord.com/invite/VcvdfAdE6R)
- [wasmCloud Slack](https://slack.wasmcloud.com/)

[new-repository-link]: https://github.com/new?template_name=concordance-gitops&template_owner=cosmonic&owner=@me&name=cosmonic-bank-account&description=Cosmonic%20bank%20account%20GitOps%20demo&visibility=public
[argo-directory-application]: https://argo-cd.readthedocs.io/en/stable/user-guide/directory/
[argo-getting-started]: https://argo-cd.readthedocs.io/en/stable/getting_started/
[concordance-banking-example]: https://github.com/cosmonic/concordance/tree/main/examples/bankaccount
[cosmo-cli-download]: https://cosmonic.com/docs/getting-started/get-the-cli
[cosmo-controller]: https://cosmonic.com/docs/kubernetes/cosmo-controller
[cosmonic]: https://app.cosmonic.com/
[`kind`]: https://kind.sigs.k8s.io/docs/user/quick-start/#installation
[`kubectl`]: https://kubernetes.io/docs/tasks/tools/#kubectl
[kubernetes-applier-actor]: https://github.com/cosmonic/kubernetes-applier/tree/main/service-applier
[kubernetes-applier-provider]: https://github.com/cosmonic/kubernetes-applier/tree/main/applier
[wadm]: https://wasmcloud.com/docs/ecosystem/wadm/model
[wasmcloud-application]: https://wasmcloud.com/docs/concepts/applications
[wormhole]: /assets/bank-account-wormhole.png
