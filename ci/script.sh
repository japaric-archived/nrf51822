set -ex

main() {
    local example=
    for example in $(ls examples/*); do
        example=$(basename $example)
        example=${example%.*}

        cross build --example $example --no-default-features --target $TARGET
        cross build --example $example --no-default-features --target $TARGET --release
    done

    for example in $(ls examples/*); do
        example=$(basename $example)
        example=${example%.*}

        cross build --example $example --target $TARGET
        cross build --example $example --target $TARGET --release
    done
}

if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
