// 5.2.29.  /proc/sysrq-trigger
// Using the echo command to write to this file, a remote root user can execute most System Request Key commands remotely as if at the local terminal. To echo values to this file, the /proc/sys/kernel/sysrq must be set to a value other than 0. For more information about the System Request Key, refer to Section 5.3.9.3, “ /proc/sys/kernel/ ”.
// Although it is possible to write to this file, it cannot be read, even by the root user.
// 
// -- https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/5/html/deployment_guide/s1-proc-topfiles#s2-proc-sysrq-trigger