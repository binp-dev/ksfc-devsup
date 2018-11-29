# Makefile for Asyn lxi support
#
# Created by alex on Thu Nov 29 16:24:17 2018
# Based on the Asyn devGpib template

TOP = .
include $(TOP)/configure/CONFIG

DIRS := configure
DIRS += $(wildcard *[Ss]up)
DIRS += $(wildcard *[Aa]pp)
DIRS += $(wildcard ioc[Bb]oot)

include $(TOP)/configure/RULES_TOP
