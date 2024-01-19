alias init := build_and_copy_to_desktop
alias room_attr_task := run_room_attribute_add_task
alias channel_unset_task := run_rate_plan_online_payment_channel_unset_task
alias operation_reason_task := run_operation_reason_add_task

crsPodIp := `kubectl get pod -n stey -l app=stey-crs -o jsonpath='{.items[0].status.podIP}'`
dcPodIp := `kubectl get pod -n stey -l app=stey-dc -o jsonpath='{.items[0].status.podIP}'`
profilePodIp := `kubectl get pod -n stey -l app=stey-profile -o jsonpath='{.items[0].status.podIP}'`
rmsPodIp := `kubectl get pod -n stey -l app=stey-rms -o jsonpath='{.items[0].status.podIP}'`
authPodIp := `kubectl get pod -n stey -l app=stey-auth -o jsonpath='{.items[0].status.podIP}'`

build_and_copy_to_desktop:
    cargo build --release
    cp ./target/release/command_line ~/Desktop/scripts

run_room_attribute_add_task ENV:
    ~/Desktop/scripts/command_line room-attribute-add --host {{crsPodIp}} -c ~/Desktop/scripts/init_data.csv -d {{ENV}}

run_rate_plan_online_payment_channel_unset_task ENV:
    ~/Desktop/scripts/command_line rate-plan-online-payment-channel-unset --host {{dcPodIp}} --origin com --channel wallet -d {{ENV}}

run_operation_reason_add_task ENV:
    ~/Desktop/scripts/command_line operation-reason-add --host {{crsPodIp}} -d {{ENV}}

run_operation_reason_remove_task ENV:
    ~/Desktop/scripts/command_line operation-reason-remove --host {{crsPodIp}} -d {{ENV}}

run_rate_plan_update_task ENV:
    ~/Desktop/scripts/command_line rate-plan-update --host {{crsPodIp}} -d {{ENV}}

run_user_identity_add_task ENV:
    ~/Desktop/scripts/command_line user-identity-add --host {{crsPodIp}} -d {{ENV}}

run_rule_add_task ENV:
    ~/Desktop/scripts/command_line rule-add --host {{rmsPodIp}} -d {{ENV}}

run_auth_permission_add_task ENV:
    ~/Desktop/scripts/command_line auth-permission-add --host {{authPodIp}} -d {{ENV}}