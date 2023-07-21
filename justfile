alias init := build_and_copy_to_desktop
alias room_attr_task := run_room_attribute_add_task

podIp := `kubectl get pod -n stey -l app=stey-crs -o jsonpath='{.items[0].status.podIP}'`

build_and_copy_to_desktop:
    cargo build --release
    cp ./target/release/command_line ~/Desktop/scripts

run_room_attribute_add_task ENV:
    ~/Desktop/scripts/command_line room-attribute-add --host {{podIp}} -c ~/Desktop/scripts/init_data.csv -d {{ENV}}
