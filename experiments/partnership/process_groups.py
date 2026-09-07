"""Race-aware POSIX child process-group cleanup."""

import os
import signal


def kill_process_group(proc):
    """Kill a live child's session without masking a live permission failure."""
    # Avoid signalling an already-reaped child's potentially reused PID.
    if proc is None or proc.poll() is not None:
        return False
    try:
        os.killpg(proc.pid, signal.SIGKILL)
    except ProcessLookupError:
        return False
    except PermissionError:
        # macOS can report EPERM for the exit race. Suppression is sound only
        # when this Popen now confirms that its child is gone.
        if proc.poll() is not None:
            return False
        raise
    return True
