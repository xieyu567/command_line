alias init := build_and_copy_to_desktop
alias room_attr_task := run_room_attribute_add_task
alias channel_unset_task := run_rate_plan_online_payment_channel_unset_task

crsPodIp := `kubectl get pod -n stey -l app=stey-crs -o jsonpath='{.items[0].status.podIP}'`
dcPodIp := `kubectl get pod -n stey -l app=stey-dc -o jsonpath='{.items[0].status.podIP}'`

build_and_copy_to_desktop:
    cargo build --release
    cp ./target/release/command_line ~/Desktop/scripts

run_room_attribute_add_task ENV:
    ~/Desktop/scripts/command_line room-attribute-add --host {{crsPodIp}} -c ~/Desktop/scripts/init_data.csv -d {{ENV}}

run_rate_plan_online_payment_channel_unset_task ENV:
    ~/Desktop/scripts/command_line rate-plan-online-payment-channel-unset --host {{dcPodIp}} --origin com --channel wallet -d {{ENV}}