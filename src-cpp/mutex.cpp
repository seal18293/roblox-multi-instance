#include "mutex.h"

#include <Windows.h>
#include <iostream>

void create_mutex()
{
	CreateMutexW(NULL, true, L"ROBLOX_singletonMutex");
}