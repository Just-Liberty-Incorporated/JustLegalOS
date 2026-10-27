# Skill: Citizen Field Observation & Lawful Evidence Collection Rules

## Purpose
Provides objective, legally accurate guidelines for gathering evidence of program fraud and documenting exterior facilities without violating state wiretapping, trespass, or privacy laws.

---

## 1. The Two Evidence Gathering Contexts

The bot must distinguish between **new exterior site observations** and **existing call/meeting recordings**:

### Context A: Conducting New Exterior Observations (DOJ Tip Rewards / WHIP)
When a citizen is gathering on-the-ground observations of a suspected ghost clinic, vacant daycare, or shuttered billing address:
1. **Public Vantage Points Only:** Remain on public sidewalks, public roadways, or open commercial parking areas. Never cross fences, enter locked employee-only areas, or touch private facility doors.
2. **Exterior Visual Evidence:** Take clear exterior photographs showing company signage, posted operating hours, address numbers, and parking lot activity from public property.
3. **No Audio Recording:** Never advise anyone to make an audio recording, covert or otherwise. Use written notes and exterior photographs from public property.
4. **Zero Staff or Patient Confrontation:** Observations must remain 100% passive. Do not interrogate employees, customers, or patients.
5. **No Social Media Posting:** Never post investigative photos, videos, or worker names to social media (X, TikTok, YouTube, Reddit). Public posting can destroy statutory reward eligibility under the False Claims Act Public Disclosure Bar (31 U.S.C. § 3730(e)(4)), alert targets to alter records, and trigger civil defamation claims.
6. **Objective Factual Logging:** Record exact dates, times, and observable physical facts (e.g. "Main entrance doors locked at 2:15 PM on Tuesday during posted hours 9 AM – 5 PM").

### Context B: Evaluating Existing Audio Recordings & Phone Call Logs
When a citizen or whistleblower *already* possesses a recorded telephone call or in-person audio with an agency representative, contractor, or employer (never advise anyone to make a new recording):
* **Do NOT blindly reject their evidence.** Evaluate the recording under governing state and federal wiretap statutes:
  1. **Federal Law & Single-Party Consent States (CO, MN, OH, TX, NY, NJ, etc.):**
     - Under **18 U.S.C. § 2511(2)(d)** and state single-party consent statutes, interception generally requires at least one participant's prior consent, provided the recording is not made for a criminal or tortious purpose.
     - **Important Caveats:** Cross-border calls (if the person on the other end is in an all-party state), in-person conversations where participants hold a reasonable expectation of privacy, or employer workplace policies create legal complexities. Software does not provide legal advice; recommend consulting licensed counsel.
  2. **12 All-Party / Two-Party Consent States (CA, FL, IL, MD, MA, MI, MT, NV, NH, PA, WA, CT):**
     - Recording confidential communications without the consent of all participants is restricted under state penal codes (e.g., *Cal. Penal Code § 632*, *Fla. Stat. § 934.03*).
     - **Public Space Exception:** Communications carried on in public streets, open government hearings, or places where participants can reasonably expect to be overheard are generally **not** "confidential communications" (*Cal. Penal Code § 632(c)*).
     - **Crime & Bribery Exception:** Statutes frequently exempt recordings made to obtain evidence reasonably believed to relate to extortion, bribery, kidnapping, or specified felony offenses (*Cal. Penal Code § 633.5*).
* **CLI Command:** Run `justlegalos-cli --json check state-traps --state <ST>` to review the governing wiretap statute before logging audio exhibits.

---

## 2. Official DOJ Pilot Program Standards

Under the DOJ Criminal Tip Rewards Pilot Program (28 U.S.C. § 530C(b)(1)(L)):
- **Truthful & Lawful Gathering:** Information provided to the Department must be truthful and complete. The Department does not authorize or condone any unlawful conduct in the gathering of information.
- **Original Information Legality:** To qualify as original information under DOJ guidelines, the information must not be obtained through illegal means or in violation of applicable federal, state, or local laws.
