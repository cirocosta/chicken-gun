HELM_FLAGS ?= --tls


setup: setup-receiver setup-transmitter

teardown: teardown-transmitter teardown-receiver


setup-%:
	helm upgrade $(HELM_FLAGS) --install --wait --values=./$*.yaml $* .

teardown-%:
	helm delete $(HELM_FLAGS) --purge $*
