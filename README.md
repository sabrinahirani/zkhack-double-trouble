## zkHack Challenge #3  
*Challenge: https://www.zkhack.dev/events/puzzle3.html*

### Relevant Background

#### Schnorr Protocol

Suppose we have a group $\mathbb{G}$ of prime order $q$ with generator $G$.
The Schnorr Protocol is a classic $\Sigma$ protocol allowing a prover $\mathcal{P}$ to convince a verifier $\mathcal{V}$ that they know a private scalar $x$ such that $P = x \cdot G$ (where the public value $P$ is known to both parties prior to the protocol execution) without revealing the value of $x$.

**Step \#1: Commitment Phase**

$\mathcal{P}$ picks $r \in_{R} \mathbb{Z}_q$. $\mathcal{P}$ computes a commitment $R = r \cdot G$.  
$\mathcal{P}$ sends $R$ to $\mathcal{V}$.

**Step \#2: Random Challenge**

$\mathcal{V}$ sends a random challenge $c \in_{R} \mathbb{Z}_q$ to $\mathcal{P}$.

**Step \#3: Response Computation**

$\mathcal{P}$ computes $s = r + c \cdot x$.  
$\mathcal{P}$ sends $s$ to $\mathcal{V}$.

**Step \#4: Verification**

$\mathcal{V}$ checks that the following holds:
$$s \cdot G \stackrel{?}{=} R + c \cdot P$$

*Since:*
$$s \cdot G = (r + c \cdot x) \cdot G = (r \cdot G) + c \cdot (x \cdot G) = R + c \cdot P$$

> **🔒 Security Assumption:**  
> The *Discrete Logarithm (DLOG) Assumption* ensures that the protocol is sound. 

---

#### Fiat-Shamir Transform

The Fiat–Shamir Transform is a method for converting interactive public-coin protocols (i.e. Schnorr) into non-interactive protocols by simulating the random public coin by hashing all previous public messages (especially messages by $\mathcal{P}$). For example, in order to convert the previously-described Schnorr Protocol into a non-interactive protocol, we might generate the random challenge $c$ by computing $c = \text{Hash}(R, P)$.

> **🔒 Security Assumption:**  
> This transform is secure in the random oracle model. 

---

#### The Double Trouble Protocol

Suppose the prover knows a private vector $\vec{a} \in \mathbb{F}_q^n$ and there is also a public vector $\vec{b} \in \mathbb{F}_q^n$.   The prover wants to prove knowledge of $\vec{a}$ and its inner product with $\vec{b}$ without revealing these values.

**Step #1: Commitment Phase**

$\mathcal{P}$ picks $\vec{r} \in_{R} \mathbb{F}_q^n$ and $\mathcal{P}$ picks $\alpha, \tau, \rho, \upsilon \in_{R} \mathbb{F}_q$.  
Then $\mathcal{P}$ computes the following:

- *a commitment to the secret vector $\vec{a}$ (offline, before public $\vec{b}$):*  
  $$C_a := \text{PedersenCommit}(\vec{a}; \alpha) = \sum_{i=1}^n a_i \cdot G_i + \alpha H$$

- *a commitment to a random vector $\vec{r}$:*  
  $$C_r := \text{PedersenCommit}(\vec{r}; \rho) = \sum_{i=1}^n r_i G_i + \rho H$$

- *a commitment to the inner product $\langle \vec{a}, \vec{b} \rangle$:*   
  $$C_1 := \text{PedersenCommit}(\langle \vec{a}, \vec{b} \rangle; \tau) = \langle \vec{a}, \vec{b} \rangle \cdot G + \tau H$$

- *a commitment to the inner product $\langle \vec{r}, \vec{b} \rangle$:*  
  $$C_2 := \text{PedersenCommit}(\langle \vec{r}, \vec{b} \rangle; \upsilon) = \langle \vec{r}, \vec{b} \rangle \cdot G + \upsilon H$$

**Step #2: Fiat–Shamir Challenge**  

$\mathcal{P}$ computes the challenge:
$$\gamma = \text{Hash}(C_a, C_r, C_1, C_2)$$

**Step #3: Response Computation**  

$\mathcal{P}$ computes the following values:
- $\vec{s} := \vec{a} + \gamma \cdot \vec{r}$
- $u := \alpha + \gamma \cdot \rho$
- $t := \tau + \gamma \cdot \upsilon$

**Step #4: Verification**

The verifier checks the following:

- *verifying linear relation on vector commitments:*
  $$\text{PedersenCommit}(\vec{s}; u) \stackrel{?}{=} C_a + \gamma \cdot C_r$$

- *verifying linear relation on inner product commitments:*
  $$\text{PedersenCommit}(\langle \vec{s}, \vec{b} \rangle; t) \stackrel{?}{=} C_1 + \gamma \cdot C_2$$

---

### The Exploit

Observe that there are only 3 public elements that use $\vec{a}$ in their computation:

1. $C_a = \sum_{i=1}^n a_i \cdot G_i + \alpha H$ 

2. $C_1 = \langle \vec{a}, \vec{b} \rangle \cdot G + \tau H$ 

3. $\vec{s} = \vec{a} + \gamma \cdot \vec{r}$

Here, (1) and (2) would require us to break the DLOG assumption to extract useful information so instead focus on (3).
Now, we know both $\vec{s}$ (from step #3) and $\gamma$ (deterministically computed from transcript). This means that the only unknown needed to determine $\vec{a}$ is $\vec{r}$.

---  
  
We proceed to find $\vec{r}$.  

*Consider the system of equations from both proofs provided:*
$$\left\{
\begin{array}{l}
\vec{a} = \vec{s}_1 - \gamma_1 \cdot \vec{r}_1 \\
\vec{a} = \vec{s}_2 - \gamma_2 \cdot \vec{r}_2
\end{array}
\right$$
$$\implies \vec{s}_1 - \vec{s}_2 = \gamma_1 \cdot \vec{r}_1 - \gamma_2 \cdot \vec{r}_2$$

*We also observe the following:*
$$C_{r2} = 2 \cdot C_{r1} \quad \Rightarrow \quad \vec{r}_2 = 2 \cdot \vec{r}_1$$

*Altogether:*
$$\implies \vec{s}_1 - \vec{s}_2 = \gamma_1 \cdot \vec{r}_1 - \gamma_2 \cdot (2 \cdot \vec{r}_1) = (\gamma_1 - 2 \cdot \gamma_2) \cdot \vec{r}_1$$

*Solving for $\vec{r}_1$:*

$$\implies \vec{r}_1 = \frac{\vec{s}_1 - \vec{s}_2}{\gamma_1 - 2 \cdot \gamma_2}$$

Using this, we can substitute back to recover the secret vector $\vec{a}$.

> **Note:** This is not a soundness attack. This breaks the zero-knowledge property by extracting the witness $\vec{a}$ from multiple valid transcripts

#### Commands

```rust
cargo run --bin verify-double-trouble
```
