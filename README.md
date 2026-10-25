    # SigmaOS-notification

    Notification daemon (d-Bus org.freedesktop.Notifications)

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/omacom/omapager

    ## Design principles applied
    - **Least Astonishment**
- **Defensive Programming**
- **Fail-Fast**
- **KISS**
- **Separation of Concerns**
- **Deterministic Testing**

    ## Usage
    Verbs: `send, history, dnd, close`

    ```
    $ sigma-notify send --summary "Updates ready" --body "12 packages" --app sigma-update
#1 [normal] sigma-update: Updates ready - 12 packages
$ sigma-notify dnd on
$ sigma-notify history --limit 5
$ sigma-notify close --id 1

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
